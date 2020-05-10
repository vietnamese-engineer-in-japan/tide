<template>
  <v-app dark>
    <v-content>
      <v-container>
        <nuxt />
      </v-container>
    </v-content>
    <v-footer>
      <div>
        <span>Powered by</span>
        <a href="https://g.co/magenta/js">magenta-js</a>,
        <a href="https://www.rust-lang.org/what/wasm">WASM by Rust</a>,
        <a href="https://nuxtjs.org">Nuxt.js</a>.
        <span>A huge thanks to</span>
        <a href="https://github.com/minh93">@minh93</a>,
        <a href="https://github.com/yuyuvn">@yuyuvn</a>.
      </div>
    </v-footer>
  </v-app>
</template>

<script>
export default {
  data() {
    return {}
  }
}
</script>
