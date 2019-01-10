<template>
	<div id="app">
		
		<h1>SQLTISE</h1>
		<!-- <button v-on:click="hello('Sucks')">Say hello from rust</button> -->
		<Reset v-if="loaded"/>
		<LoadCsvFile v-if="!loaded"/>
		<SelectStatements v-if="loaded && !statements.done"/>
		<ColumnSelections v-if="statements.done && !column_selections.done"/>
		<SolveErrors v-if="errors.length > 0 && column_selections.done"/>
		<div v-if="errors.length === 0 && column_selections.done">
			<button class="generate-button">GENERATE</button>	
		</div>
		<div v-if="debug">
			<h3>Debug:</h3>
			<div><pre>Columns: {{columns}}</pre></div>
			<div><pre>Statements: {{statements}}</pre></div>
			<div><pre>Column Selections: {{column_selections}}</pre></div>
			<div><pre>Solved errors: {{error_solutions}}</pre></div>
		</div>
	</div>
</template>

<script>
import LoadCsvFile from './components/LoadCsvFile.vue';
import Reset from './components/Reset.vue';
import SelectStatements from './components/SelectStatements.vue';
import ColumnSelections from './components/ColumnSelections.vue';
import SolveErrors from './components/SolveErrors.vue';
import { mapState } from 'vuex'

export default {
	name: 'app',
	computed: {
		...mapState([
			'columns',
			'loaded',
			'statements',
			'debug',
			'column_selections',
			'errors',
			'error_solutions'
		]),
		// other properties
	},
	components: {
		LoadCsvFile,
		Reset,
		SelectStatements,
		ColumnSelections,
		SolveErrors,
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

.generate-button{
	background-color: #277554;
    border: none;
    color: white;
    font-size: 1.25em;
    padding: 8px;
    height: 60px;
    width: 240px;
	margin-top: 16px;
}
</style>
