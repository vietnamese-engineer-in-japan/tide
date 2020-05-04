<template>
  <div ref="container" style="overflow-x: hidden">
    <v-toolbar flat height="90">
      <v-spacer />
      <v-btn outlined fab small color="light-blue" @click="stop">
        <v-icon>mdi-stop</v-icon>
      </v-btn>
      <v-btn outlined fab color="light-blue" @click="play">
        <v-icon large>mdi-play</v-icon>
      </v-btn>
      <v-btn outlined fab small color="light-blue" @click="pause">
        <v-icon>mdi-pause</v-icon>
      </v-btn>
      <v-spacer />
    </v-toolbar>
    <v-toolbar flat height="30">
      <v-slider v-model="volume" max="1" step="0.1" />
    </v-toolbar>
    <v-toolbar flat height="90">
      <v-progress-linear v-model="progress" height="40" color="light-blue" />
    </v-toolbar>
    <div id="notation" :style="notationTransform" />
  </div>
</template>

<script>
import colors from 'vuetify/es5/util/colors'
import { Player } from 'midi-player-js'
import Soundfont from 'soundfont-player'
import SVG from 'svg.js'

export default {
  props: {
    player: {
      type: Player,
      default: null
    }
  },
  data() {
    return {
      volume: 0.5,
      progress: 0,
      scale: 0.5,
      width: 0
    }
  },
  computed: {
    notationTransform() {
      const x =
        this.width / 2 -
        (this.progress === 0 ? 0 : this.player.tick * this.scale)
      return `transform: translateX(${x}px)`
    }
  },
  mounted() {
    const AudioContext =
      window.AudioContext || window.webkitAudioContext || false
    const context = new AudioContext()
    const loud = 4
    // Play note
    Soundfont.instrument(new AudioContext(), 'acoustic_grand_piano').then(
      (instrument) => {
        this.player.on('playing', () => {
          this.progress = 100 - this.player.getSongPercentRemaining()
        })
        this.player.on('midiEvent', (event) => {
          if (event.name === 'Note on' && event.velocity > 0) {
            instrument.play(event.noteName, context.currentTime, {
              gain: (event.velocity / 100) * this.volume * loud
            })
            SVG.select('.tick').fill(colors.indigo.base)
            SVG.select(`.tick${event.tick}`).fill(colors.lightBlue.base)
          }
        })
      }
    )
    // Draw notation
    const length = this.player.totalTicks * this.scale
    const height = 400
    const notation = SVG('notation').size(length, height)
    const lineWidth = 4
    const lineCount = 5
    const lineSpace = 20
    const noteHeight = 40
    const topLineY =
      lineWidth / 2 + (height - (lineCount - 1) * lineSpace - noteHeight) * 0.25
    for (let i = 0; i < lineCount; i++) {
      const y = topLineY + i * lineSpace
      notation.line(lineWidth / 2, y, length - lineWidth / 2, y).stroke({
        color: colors.brown.base,
        width: lineWidth,
        linecap: 'round'
      })
    }
    const events = this.player.getEvents().reduce(function(total, current) {
      total.push(...current)
      return total
    })
    const diff = function(note1, note2) {
      return (
        (note1.match(/\d/g)[0] - note2.match(/\d/g)[0]) * 7 +
        (note1.charCodeAt(0) - note2.charCodeAt(0))
      )
    }
    const topNoteName = 'F5' // 'F5' note lies at top line
    events.forEach((event) => {
      if (event.name === 'Note on' && event.velocity > 0) {
        const x = event.tick * this.scale
        notation
          .text(event.noteName)
          .attr('class', `tick tick${event.tick}`)
          .font({
            fill: colors.indigo.base,
            size: 20,
            style: 'font-weight: bold'
          })
          .center(
            x,
            topLineY - (diff(event.noteName, topNoteName) * lineSpace) / 2
          )
      }
    })
    this.width = this.$refs.container.clientWidth
  },
  methods: {
    play() {
      if (this.player.isPlaying()) {
      } else {
        this.player.play()
      }
    },
    pause() {
      this.player.pause()
    },
    stop() {
      this.player.stop()
      this.progress = 0
    }
  }
}
</script>
