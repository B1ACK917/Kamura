<template>
  <KamuraMain/>
  <footer style="position: absolute; bottom: 0; width: 100%">
    Powered by Kamura-Engine
  </footer>
</template>

<script>
import KamuraMain from './components/KamuraMain.vue'

export default {
  name: 'App',
  components: {
    KamuraMain
  }
}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
