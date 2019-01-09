<template>
	<div id="app">
		
		<h1>SQLTISE</h1>
		<!-- <button v-on:click="hello('Sucks')">Say hello from rust</button> -->
		<Reset v-if="loaded"/>
		<LoadCsvFile v-if="!loaded"/>
		<SelectStatements v-if="loaded && !statements.done"/>
		<div>
			<h3>Debug:</h3>
			<div><pre>Columns: {{columns}}</pre></div>
			<div><pre>Statements: {{statements}}</pre></div>
		</div>
	</div>
</template>

<script>
import LoadCsvFile from './components/LoadCsvFile.vue';
import Reset from './components/Reset.vue';
import SelectStatements from './components/SelectStatements.vue';
import { mapState } from 'vuex'

export default {
	name: 'app',
	computed: {
		...mapState([
			'columns',
			'loaded',
			'statements'
		]),
		// other properties
	},
	components: {
		LoadCsvFile,
		Reset,
		SelectStatements,
	},
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
