<template>
	<div class="column-selections">
		<h3>Columns</h3>
		
		<div class="statement" v-for="(statement, index) in statements.value" :key="index" :value="statement.id">
			<div v-if="statement.type == 'Custom'">
				<h3>{{statement.name}} - Custom</h3>
				<input v-model="statement.custom" class="custom-input"/>
			</div>
			<div v-if="statement.type != 'Custom'">
				<div v-if="!statement.column_selections || statement.column_selections.value.length === 0">
					<h5>There are no columns selected</h5>
				</div>
				<div v-if="statement.column_selections">
					<h3>{{statement.name}} - ({{statement.table}})</h3>
					<div class="columns-header" v-if="statement.column_selections && statement.column_selections.value.length > 0">
						<div>Source</div>
						<div>Data</div>
						<div>Type</div>
						<div id="small-column">Use Source Name</div>
						<div>Destination Column</div>
						<div id="small-column">Delete</div>
					</div>
					<div class="columns" v-for="(column, index2) in statement.column_selections.value" :key="index2">
						<select v-model="column.source" class="column-select">
							<option>CSV</option>
							<option>FreeText</option>
						</select>
						<input v-if="!column.source" class="column-input" disabled/>
						<input type="text" class="column-input" v-model="column.data" v-show="column.source == 'FreeText'"/>
						<select v-model="column.column" class="column-select" v-if="column.source == 'CSV'">
							<option v-for="(option, index3) in columns" :key="index3" :value="option.index">{{option.name}}</option>
						</select>
						<select v-model="column.type" class="column-select">
							<option>Int</option>
							<option>Float</option>
							<option>Date</option>
							<option>DateTime</option>
							<option>VarChar</option>
							<option>PreFormatted</option>
						</select>
						<div id="small-column" class="column-checkbox">
							<input type="checkbox" id="checkbox" v-model="column.use_source" v-if="column.source != 'CSV'" disabled/>
							<input type="checkbox" id="checkbox" v-model="column.use_source" v-if="column.source == 'CSV'"/>
						</div>
						<input v-model="column.name" class="column-input"/>
						<button id="small-column" class="remove-button" v-on:click="remove({index, index2})"><i class="fa fa-trash"></i></button>
					</div>
					<button class="columns-button" v-on:click="add(index)">Add Column <i class="fa fa-plus add-icon"></i></button>
					<table class="where-clause" v-if="statement.type === 'Update'">
						<thead>
							<tr>
								<th></th>
								<th>DB Column</th>
								<th></th>
								<th>Source Column</th>
								<th>Type</th>
							</tr>	
						</thead>
						<tbody >
							<tr v-for="(where, whereindex) in statement.where_selections" :key="whereindex">
								<td class="where-text" v-if="whereindex === 0">WHERE</td>
								<td class="where-text" v-if="whereindex != 0">AND</td>
								<td>
									<input v-model="where.key" class="column-input"/>
								</td>
								<td class="equals">=</td>
								<td>
									<select v-model="where.value" class="column-select">
										<option v-for="(option, index4) in columns" :key="index4" :value="option.index">{{option.name}}</option>
									</select>
								</td>
								<td>
									<select v-model="where.type" class="column-select">
										<option>Int</option>
										<option>Float</option>
										<option>Date</option>
										<option>DateTime</option>
										<option>VarChar</option>
									</select>
								</td>
								<td>
									<button id="small-column" class="remove-button" v-on:click="remove_where({index, whereindex})"><i class="fa fa-trash"></i></button>
								</td>
							</tr>	
						</tbody>
					</table>
					<button class="columns-button" v-on:click="add_where(index)" v-if="statement.type === 'Update'">Add Condition <i class="fa fa-plus add-icon"></i></button>
				</div>
			</div> 
		</div>	
		<button class="columns-button" v-on:click="done" v-if="columns_complete(statements)">Done <i class="fa fa-check add-icon"></i></button>
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
			'ADD_WHERE',
			'REMOVE_WHERE',
		]),
		add: function (index) {
			this.ADD_COLUMN(index);
		},
		done: function () {
			this.DONE_ADDING_COLUMNS();
		},
		columns_complete: function(statements) {
			let completed;
			statements.value.forEach(statement => {
				if (statement.type == 'Custom'){
					completed = statement.custom.length > 0; 
				}

				let columns = statement.column_selections.value;
				if (!columns || columns.length === 0) {
					return false;
				}

				if (statement.type === 'Update' && statement.where_selections.some(w => (!w.key || w.value == undefined || w.type == undefined ))) {
					return false;
				}

				completed =  columns.every((s) => {
						return (s.column != undefined || s.source == 'FreeText') && !!s.type && (!!s.name || s.use_source) && !(!!s.name && s.use_source)
					});	

				if (!completed)	{
					return completed;
				}
			});
			return completed;
		},
		remove: function(arg) {
			this.REMOVE_COLUMN(arg);
		},
		remove_where: function(arg) {
			this.REMOVE_WHERE(arg);
		},
		add_where: function(index) {
			this.ADD_WHERE(index);
		}
		// other methods
	},
	computed: {
		...mapState([
			'columns',
			'statements',
			'selected_column'
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
	cursor: pointer;
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

.statement {
	border: 1px solid black;
	padding: 16px;
	margin-bottom: 16px;
}

.where-clause {
	margin-top: 16px;
}

.where-text {
	padding-right: 8px;
}

.equals {
	padding-right: 15px;
}

.custom-input{
	margin-right: 16px;
	font-size: 16px;
	padding: 8px;
	min-width: 800px;
	width: 98%;
}

.custom-text{
	margin-bottom: 10px;
	width: 800px;
}
</style>
