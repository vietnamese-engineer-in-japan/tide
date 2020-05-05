<template>
  <v-container>
    <v-row>
      <v-col :cols="6">
        <v-file-input v-model="file" :disabled="isProcessing"></v-file-input>
      </v-col>
      <v-col :cols="3">
        <v-btn :disabled="isProcessing" @click="process">
          {{ isProcessing ? 'Processing...' : 'Process' }}
        </v-btn>
      </v-col>
      <v-col :cols="3">
        <v-btn v-if="isDone" @click="save">Save</v-btn>
      </v-col>
    </v-row>
    <v-flex>
      <player-component v-if="isDone" :player="player" />
    </v-flex>
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
    return {
      file: null,
      isProcessing: false,
      isDone: false,
      midi: null,
      player: new Player()
    }
  },
  methods: {
    async process(event) {
      this.isProcessing = true
      this.isDone = false
      let result = await wasm.read_file(this.file)
      const array = new Uint8Array(result)
      // See: https://github.com/grimmdude/MidiPlayerJS/blob/master/src/player.js#L103
      if (Utils.bytesToLetters(array.subarray(0, 4)) !== 'MThd') {
        const audio = await wasm.decode_audio(result)
        const rate = audio.sampleRate
        const bufferLength = rate * 10
        const data = new Float32Array(bufferLength)
        const context = new OfflineAudioContext(1, rate, rate)
        const buffer = context.createBuffer(2, bufferLength, rate)
        const sequences = []
        for (let i = 0; i < Math.ceil(audio.length / bufferLength); i++) {
          for (let j = 0; j < audio.numberOfChannels; j++) {
            audio.copyFromChannel(data, j, i * bufferLength)
            buffer.copyToChannel(data, j, 0)
          }
          sequences.push(await this.model.transcribeFromAudioBuffer(buffer))
        }
        const sequence = mm.sequences.concatenate(sequences)
        this.midi = mm.sequenceProtoToMidi(sequence)
        result = this.midi.buffer
      } else {
        this.midi = array
      }
      this.player.loadArrayBuffer(result)
      this.isProcessing = false
      this.isDone = true
    },
    save() {
      const name = this.file.name.split('.')[0]
      saveAs(new File([this.midi], `${name}.mid`))
    }
  }
}
</script>
