<template>
	<div id="app">

		<h1>SQLTISE</h1>
		<Reset v-if="loaded"/>

		<div class="transition-wrapper">
			<transition name="zoom">
				<LoadCsvFile v-if="!loaded"/>
			</transition>
			<transition name="zoom">
				<SelectStatements v-if="loaded && !statements.done"/>
			</transition>
			<transition name="zoom">
				<ColumnSelections v-if="statements.done && !statements.columnSelectionsDone"/>
			</transition>
			<transition name="zoom">
				<SolveErrors v-if="errors.length > 0 && statements.columnSelectionsDone"/>
			</transition>
			<transition name="zoom">
				<div v-if="errors.length === 0 && statements.done && statements.columnSelectionsDone && downloads.length === 0">
					<button class="generate-button" v-on:click="generate">GENERATE</button>
				</div>
			</transition>
		</div>

		<div class="generated-file" v-for="(dl, index) in downloads" :key="index">
			<a :href="dl.url" :download="dl.name">download {{dl.name}}</a>
		</div>

		<div v-if="debug">
			<h3>Debug:</h3>
			<div><pre>Columns: {{columns}}</pre></div>
			<div><pre>Statements: {{statements}}</pre></div>
			<!-- <div><pre>Column Selections: {{statements.column_selections}}</pre></div> -->
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
import { mapState, mapMutations } from 'vuex'


export default {
	name: 'app',
	computed: {
		...mapState([
			'columns',
			'loaded',
			'statements',
			'debug',
			'errors',
			'error_solutions',
			'downloads',
		]),
		// other properties
	},
	methods: {
		...mapMutations([
			'GENERATE_SQL',
		]),
		generate: function() {
			this.GENERATE_SQL();
		}
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
/* body {
	display: inline-block;
	vertical-align: middle;
	max-width: 38rem;
}; */

#app {
	font-family: 'Avenir', Helvetica, Arial, sans-serif;
	-webkit-font-smoothing: antialiased;
	-moz-osx-font-smoothing: grayscale;
	color: #2c3e50;
	padding: 10px 50px;

	/* position: fixed;
	top: 50%;
	left: 50%;
	-webkit-transform: translate(-50%, -50%);
	transform: translate(-50%, -50%);
	text-align: center; */
}

#app > h1 {
	font-size: 50px;
}

.transition-wrapper {
	display: flex;
	flex-direction: row;

	/* justify-content: center;
	align-items: center; */
}

.zoom-enter-active {
	animation: slideInLeft .5s;
}

/* .zoom-leave-active {
	animation: slideOutLeft 0s;
} */

.generate-button{
	background-color: #277554;
	border: none;
	color: white;
	font-size: 1.25em;
	padding: 8px;
	height: 60px;
	width: 240px;
	margin-top: 16px;
	cursor: pointer;
}

.generate-button:hover {
	background-color: #499273;
}

.generated-file{
	margin-top: 16px;
	width: 240px;
	height: 60px;
	background-color: #2c3e50;
	cursor: pointer;
	display: flex;
	justify-content: center;
	align-items: center;
}

.generated-file:hover {
	background-color:  #4c5e70;
}

.generated-file a{
	text-decoration: none;
	color: white;
	padding: 8px;
	text-transform: uppercase;
	text-align: center;
}

</style>
