<template>
	<div id="app">

		<h1>SQLTISE</h1>
		<!-- <button v-on:click="hello('Sucks')">Say hello from rust</button> -->
		<Reset v-if="loaded"/>

		<div class="transition-wrapper">
			<transition name="zoom">
				<LoadCsvFile v-if="!loaded"/>
			</transition>
			<transition name="zoom">
				<SelectStatements v-if="loaded && !statements.done"/>
			</transition>
			<transition name="zoom">
				<ColumnSelections v-if="statements.done && !column_selections.done"/>
			</transition>
			<transition name="zoom">
				<SolveErrors v-if="errors.length > 0 && column_selections.done"/>
			</transition>
			<transition name="zoom">
				<div v-if="errors.length === 0 && column_selections.done">
					<button class="generate-button" v-on:click="generate">GENERATE</button>
				</div>
			</transition>
			<div v-if="downloads.length > 0">
				<a v-for="(dl, index) in downloads" :key="index" :href="dl.url" :download="dl.name">{{dl.name}}</a>
			</div>
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
import { mapState, mapMutations } from 'vuex'


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
	padding: 0px 30px;
		position: fixed;
	top: 50%;
	left: 50%;
	-webkit-transform: translate(-50%, -50%);
	transform: translate(-50%, -50%);
	text-align: center;
}

.transition-wrapper {
	display: flex;
	flex-direction: row;
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
}
</style>
