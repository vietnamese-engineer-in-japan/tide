<template>
  <v-app dark>
    <v-content>
      <v-container>
        <nuxt />
      </v-container>
    </v-content>
    <v-footer>
      <div>
        Powered by
        <a href="https://g.co/magenta/js">magenta-js</a>,
        <a href="https://www.rust-lang.org/what/wasm">WASM by Rust</a>,
        <a href="https://nuxtjs.org">Nuxt.js</a>
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
