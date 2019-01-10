<template>
	<div class="column-selections">
		<h3>Columns</h3>
		<div v-if="column_selections.length === 0">
			<h5>There are no columns selected</h5>
		</div>
		<div class="columns-header" v-if="column_selections.value.length > 0">
			<div>Source Column</div>
			<div>Type</div>
			<div>Statement</div>
			<div id="small-column">Use Source Name</div>
			<div>Destination Column</div>
			<div id="small-column">Delete</div>
		</div>
		<div class="columns" v-for="(column, index) in column_selections.value" :key="index">
			<select v-model="column.column" class="column-select">
				<option v-for="(option, index) in columns" :key="index" :value="option.index">{{option.name}}</option>
			</select>
			<select v-model="column.type" class="column-select">
				<option>Int</option>
				<option>Float</option>
				<option>Date</option>
				<option>DateTime</option>
				<option>VarChar</option>
			</select>
			<select v-model="column.statement_id" class="column-select">
				<option v-for="(option, index) in statements.value" :key="index" :value="option.id">{{option.name}} - ({{option.table}})</option>
			</select>
			<div id="small-column" class="column-checkbox">
				<input type="checkbox" id="checkbox" v-model="column.use_source"/>
			</div>
			<input v-model="column.name" class="column-input"/>
			<button id="small-column" class="remove-button" v-on:click="remove(index)"><i class="fa fa-trash"></i></button>
		</div>
		<button class="columns-button" v-on:click="add">Add Column<i class="fa fa-plus add-icon"></i></button>
		<button class="columns-button" v-on:click="done" v-if="columns_complete(column_selections.value)">Done<i class="fa fa-check add-icon"></i></button>
	</div>
</template>

<script>
import { mapState, mapMutations } from 'vuex'

export default {
	name: 'ColumnSelections',
	methods: {
		...mapMutations([
			'ADD_COLUMN',
			'DONE_ADDING_COLUMNS',
			'REMOVE_COLUMN',
		]),
		add: function () {
			this.ADD_COLUMN();
		},
		done: function () {
			this.DONE_ADDING_COLUMNS();
		},
		columns_complete: function(columns) {
			if (!columns || columns.length === 0){
				return false;
			}
			return columns.every((s) => {
					return s.column != undefined && s.statement_id != undefined  && !!s.type && (!!s.name || s.use_source) && !(!!s.name && s.use_source)
				});
		},
		remove: function(index) {
			this.REMOVE_COLUMN(index);
		},
		// other methods
	},
	computed: {
		...mapState([
			'columns',
			'column_selections',
			'statements',
		]),
		// other properties
	},
}
</script>

<style scoped>
.column-selections{
	margin-top: 20px;
}


.columns-button{
	background-color: #277554;
	border: none;
	color: white;
	padding: 8px;
	margin-right: 16px;
}

.columns-button:hover {
	background-color: #499273;
}

.column-select{
	margin-right: 16px;
	padding: 10px;
	font-size: 16px;
	min-width: 225px;
	max-width: 225px;
}

.columns-header {
	margin: 16px 0px;
	width: 800px;
	display: flex;
	align-items: center;
}

.columns-header div{
	margin-right: 16px;
	text-align: center;
	min-width: 225px;
	max-width: 225px;
	font-weight: 700;
}

.columns{
	margin: 16px 0px;
	display: flex;
}

.column-input{
	margin-right: 16px;
	font-size: 16px;
	padding: 8px;
	min-width: 207px;
	max-width: 207px;
}

.remove-button{
	background-color: white;
	border: none;
	color: #630A0C;
	font-size: 1.25em;
	min-width: 225px;
	max-width: 225px;
	text-align: center;
}

.remove-button:hover {
	color: #A8383B;
	cursor: pointer;
}

#small-column{
	text-align: center;
	min-width: 70px;
	max-width: 70px;
	width: 70px;
}

.column-checkbox{
	margin-right: 16px;
	display: flex;
	justify-content: center;
	align-items: center;
}
</style>
