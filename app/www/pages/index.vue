<template>
  <v-container>
    <v-row>
      <v-col :cols="3">
        <v-select v-model="type" :items="types"></v-select>
      </v-col>
      <v-col :cols="5">
        <v-text-field v-if="isYoutube" v-model="youtubeUrl"></v-text-field>
        <v-file-input
          v-else
          v-model="file"
          :disabled="isProcessing"
        ></v-file-input>
      </v-col>
      <v-col :cols="2">
        <v-btn :disabled="isProcessing" @click="process">
          {{
            isProcessing
              ? `Processing${progress ? ` (${progress})` : ''}`
              : 'Process'
          }}
        </v-btn>
      </v-col>
      <v-col :cols="2">
        <v-btn v-if="isSuccessful" @click="save">Save</v-btn>
      </v-col>
    </v-row>
    <v-flex>
      <player-component v-if="isSuccessful" :player="player" />
    </v-flex>
    <v-dialog v-model="messageDialog" max-width="300">
      <v-card>
        <v-card-title>Message</v-card-title>
        <v-card-text>{{ messageText }}</v-card-text>
      </v-card>
    </v-dialog>
  </v-container>
</template>

<script>
import * as mm from '@magenta/music'
import * as wasm from 'tide'
import { saveAs } from 'file-saver'
import { Player, Utils } from 'midi-player-js'
import PlayerComponent from '~/components/Player.vue'

export default {
  components: {
    PlayerComponent
  },
  async asyncData() {
    const model = new mm.OnsetsAndFrames(
      'https://storage.googleapis.com/magentadata/js/checkpoints/transcription/onsets_frames_uni'
    )
    await model.initialize()
    return {
      model
    }
  },
  data() {
    const types = ['Youtube URL', 'Audio file']
    return {
      // Read-only
      type: types[0],
      types,
      youtubeUrl: '',
      file: null,
      // Process
      fileName: '',
      progress: '',
      isProcessing: false,
      isSuccessful: false,
      midi: null,
      messageDialog: false,
      messageText: '',
      // Playing
      player: new Player()
    }
  },
  computed: {
    isYoutube() {
      return this.type === this.types[0]
    }
  },
  methods: {
    async process(event) {
      const showDialog = (messageText) => {
        this.messageText = messageText
        this.messageDialog = true
        this.isSuccessful = false
        this.isProcessing = false
      }
      this.fileName = ''
      this.progress = 'reading'
      this.isProcessing = true
      this.isSuccessful = false
      this.midi = null
      this.messageDialog = false
      this.messageText = ''
      let file = null
      if (this.isYoutube) {
        let url = null
        try {
          url = new URL(this.youtubeUrl)
        } catch (e) {
          return showDialog('Invalid URL')
        }
        const videoId = new URLSearchParams(url.search).get('v')
        const apiUrl = 'https://oggy.metalwhale.dev:8000'
        const streamUrl = `${apiUrl}/youtube_audio?video_id=${videoId}`
        const infoUrl = `${apiUrl}/youtube_info?video_id=${videoId}`
        file = await this.$axios.$get(streamUrl, { responseType: 'blob' })
        const info = await this.$axios.$get(infoUrl)
        if (info.info) {
          this.fileName = info.info.title
        }
      } else {
        file = this.file
        this.fileName = file.name.split('.')[0]
      }
      let fileBuffer = await wasm.read_file(file)
      const array = new Uint8Array(fileBuffer)
      // See: https://github.com/grimmdude/MidiPlayerJS/blob/master/src/player.js#L103
      if (Utils.bytesToLetters(array.subarray(0, 4)) !== 'MThd') {
        let audio = null
        try {
          audio = await wasm.decode_audio(fileBuffer)
        } catch (e) {
          return showDialog('Error occurred while decoding')
        }
        if (audio) {
          const rate = audio.sampleRate
          const bufferLength = rate * 10
          const data = new Float32Array(bufferLength)
          const context = new OfflineAudioContext(1, rate, rate)
          const buffer = context.createBuffer(2, bufferLength, rate)
          const sequences = []
          const sequencesCount = Math.ceil(audio.length / bufferLength)
          for (let i = 0; i < sequencesCount; i++) {
            const length = i * bufferLength
            this.progress = `${i + 1}/${sequencesCount} sequences`
            for (let j = 0; j < audio.numberOfChannels; j++) {
              audio.copyFromChannel(data, j, length)
              buffer.copyToChannel(data, j, 0)
            }
            sequences.push(await this.model.transcribeFromAudioBuffer(buffer))
          }
          const sequence = mm.sequences.concatenate(sequences)
          this.midi = mm.sequenceProtoToMidi(sequence)
          fileBuffer = this.midi.buffer
        }
      } else {
        this.midi = array
      }
      this.player.loadArrayBuffer(fileBuffer)
      this.isSuccessful = true
      this.isProcessing = false
    },
    save() {
      saveAs(new File([this.midi], `${this.fileName}.mid`))
    }
  }
}
</script>
