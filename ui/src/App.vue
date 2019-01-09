<template>
	<div id="app">
		<h1>SQLTISE</h1>
		<!-- <button v-on:click="hello('Sucks')">Say hello from rust</button> -->
		<LoadCsvFile/>
	</div>
</template>

<script>
//import HelloWorld from './components/HelloWorld.vue'
import LoadCsvFile from './components/LoadCsvFile.vue'
var csv2sql = import('./csv2sql/csv2sql')

export default {
	name: 'app',
	components: {
		LoadCsvFile
	},
	methods: {
		hello: (f) => {
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
	color: #2c3e50;
	padding: 0px 30px;
}
</style>
