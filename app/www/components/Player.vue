<template>
  <div ref="container" style="overflow-x: hidden">
    <v-toolbar flat height="90">
      <v-row align="center">
        <v-col :cols="5" align="end">
          <v-btn outlined fab small color="light-blue" @click="stop">
            <v-icon>mdi-stop</v-icon>
          </v-btn>
        </v-col>
        <v-col :cols="2" align="center">
          <v-btn outlined fab color="light-blue" @click="play">
            <v-icon large>
              {{ player.isPlaying() ? 'mdi-pause' : 'mdi-play' }}
            </v-icon>
          </v-btn>
        </v-col>
        <v-col :cols="5">
          <v-slider
            v-model="volume"
            append-icon="mdi-volume-high"
            prepend-icon="mdi-volume-low"
            max="10"
            step="0.1"
          />
        </v-col>
      </v-row>
    </v-toolbar>
    <v-toolbar flat height="60">
      <v-progress-linear
        :value="progress"
        height="20"
        color="light-blue"
        @click="setProgress($event)"
      />
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
      volume: 1,
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
    const eventFilter = function(event) {
      return event.name === 'Note on' && event.velocity > 0
    }
    this.player.getSongPercent = function() {
      const songTime = (this.getCurrentTick() / this.division / this.tempo) * 60
      return (songTime / this.getSongTime()) * 100
    }
    // Play note
    Soundfont.instrument(new AudioContext(), 'acoustic_grand_piano').then(
      (instrument) => {
        this.player.on('playing', () => {
          this.progress = Math.round(this.player.getSongPercent())
        })
        let lastTick = null
        this.player.on('midiEvent', (event) => {
          if (eventFilter(event)) {
            instrument.play(event.noteName, context.currentTime, {
              gain: (event.velocity / 100) * this.volume
            })
            if (lastTick) {
              SVG.select(`.tick${lastTick}`).fill(colors.indigo.base)
            }
            SVG.select(`.tick${event.tick}`).fill(colors.lightBlue.base)
            lastTick = event.tick
          }
        })
      }
    )
    // List events
    const events = this.player
      .getEvents()
      .reduce(function(total, current) {
        total.push(...current)
        return total
      })
      .filter(eventFilter)
    const diff = function(note1, note2) {
      return (
        (note1.match(/\d/g)[0] - note2.match(/\d/g)[0]) * 7 +
        (note1.charCodeAt(0) - note2.charCodeAt(0))
      )
    }
    let highestNote = null
    let lowestNote = null
    events.forEach((event) => {
      if (!highestNote || diff(highestNote, event.noteName) < 0) {
        highestNote = event.noteName
      }
      if (!lowestNote || diff(lowestNote, event.noteName) > 0) {
        lowestNote = event.noteName
      }
    })
    // Draw notation
    const lineWidth = 4
    const lineCount = 5
    const lineSpace = 20
    const topNoteName = 'F5' // 'F5' note lies at top line
    const space = function(note1, note2) {
      return (diff(note1, note2) * lineSpace) / 2
    }
    const length = this.player.totalTicks * this.scale
    const margin = 20
    const height = space(highestNote, lowestNote) + margin * 2
    const notation = SVG('notation').size(length, height)
    const topLineY = space(highestNote, topNoteName) + margin
    for (let i = 0; i < lineCount; i++) {
      const y = topLineY + i * lineSpace
      notation.line(lineWidth / 2, y, length - lineWidth / 2, y).stroke({
        color: colors.brown.base,
        width: lineWidth,
        linecap: 'round'
      })
    }
    events.forEach((event) => {
      const x = event.tick * this.scale
      notation
        .text(event.noteName)
        .attr('class', `tick tick${event.tick}`)
        .font({
          fill: colors.indigo.base,
          size: lineSpace,
          style: 'font-weight: bold'
        })
        .center(x, topLineY - space(event.noteName, topNoteName))
    })
    this.width = this.$refs.container.clientWidth
  },
  methods: {
    play() {
      if (this.player.isPlaying()) {
        this.player.pause()
      } else {
        this.player.play()
      }
    },
    stop() {
      this.player.stop()
      this.progress = 0
    },
    setProgress(event) {
      const position = event.offsetX
      const bar = document.querySelector('.v-progress-linear')
      const width = bar.clientWidth
      const progress = (position / width) * 100
      if (this.player.isPlaying()) {
        this.player.skipToPercent(progress)
        this.player.play()
      }
    }
  }
}
</script>
