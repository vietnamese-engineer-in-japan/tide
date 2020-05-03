<template>
  <v-layout column justify-center align-center>
    <v-flex xs12 sm8 md6>
      <div class="text-center">
        <logo />
        <vuetify-logo />
      </div>
      <v-card>
        <v-card-title class="headline">
          Welcome to the Vuetify + Nuxt.js template
        </v-card-title>
        <v-card-text>
          <p>
            Vuetify is a progressive Material Design component framework for
            Vue.js. It was designed to empower developers to create amazing
            applications.
          </p>
          <p>
            For more information on Vuetify, check out the
            <a href="https://vuetifyjs.com" target="_blank"> documentation </a>.
          </p>
          <p>
            If you have questions, please join the official
            <a href="https://chat.vuetifyjs.com/" target="_blank" title="chat">
              discord </a
            >.
          </p>
          <p>
            Find a bug? Report it on the github
            <a
              href="https://github.com/vuetifyjs/vuetify/issues"
              target="_blank"
              title="contribute"
            >
              issue board </a
            >.
          </p>
          <p>
            Thank you for developing with Vuetify and I look forward to bringing
            more exciting features in the future.
          </p>
          <div class="text-xs-right">
            <em><small>&mdash; John Leider</small></em>
          </div>
          <hr class="my-3" />
          <a href="https://nuxtjs.org/" target="_blank">
            Nuxt Documentation
          </a>
          <br />
          <a href="https://github.com/nuxt/nuxt.js" target="_blank">
            Nuxt GitHub
          </a>
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn color="primary" nuxt to="/inspire">
            Continue
          </v-btn>
          <input type="file" @change="read_file" />
        </v-card-actions>
      </v-card>
    </v-flex>
  </v-layout>
</template>

<script>
import * as mm from '@magenta/music'
import { saveAs } from 'file-saver'
import * as wasm from 'tide'
import Logo from '~/components/Logo.vue'
import VuetifyLogo from '~/components/VuetifyLogo.vue'

export default {
  components: {
    Logo,
    VuetifyLogo
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
  methods: {
    async read_file(event) {
      const result = await wasm.read_file(event.target.files[0])
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
      saveAs(new File([mm.sequenceProtoToMidi(sequence)], 'transcription.mid'))
    }
  }
}
</script>
