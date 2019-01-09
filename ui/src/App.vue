<template>
  <div id="app">
    <button v-on:click="bacon('Sucks')">Say hello from rust</button>
    <HelloWorld msg="Hello World"/>
  </div>
</template>

<script>
import HelloWorld from './components/HelloWorld.vue'
var csv2sql = import('./csv2sql/csv2sql')

export default {
  name: 'app',
  components: {
    HelloWorld
  },
  methods: {
    bacon: (f) => {
      csv2sql._v.greet(f);
    }
  }
}
</script>

<style>
#app {
  font-family: 'Avenir', Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
