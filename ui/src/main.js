/* eslint-disable */
import Vue from 'vue'
import Vuex from 'vuex'
import App from './App.vue'

Vue.config.productionTip = false
Vue.use(Vuex);
var csv2sql = import('./csv2sql/csv2sql');

var default_state = function() {
	return {
		columns: [{name:"StateCode",index:0}],
		raw_csv: null,
		loaded: false,
		statements: {
			done: false,
			columnSelectionsDone: false,
			nextid: 0,
			value: [
			],
		},
		error_solutions: [],
		errors: [],
		downloads: [],
	}
};

var store = new Vuex.Store({
	state: Object.assign({}, default_state()),
	mutations: {
		LOAD_CSV: (state, data) => {
			csv2sql
				.then(m => {
					var result = m.get_columns(data);
					if (result) {
						state.columns = result.columns;
						state.raw_csv = data;
						state.loaded = true;
					}
				})
				.catch(console.error);

		},
		RESET: (state) => {
			//state_stack.push(state);
			var new_state = Object.assign(state, default_state());
			state = new_state;
		},
		ADD_STATEMENT: (state) => {
			var id = state.statements.nextid++;
			var new_statement = {
				 column_selections: {
					 value: [{
						 column: null,
						 name: null,
						 type: null,
						 use_source: false }],
					done: false },
				 id: id,
				 type: null,
				 name: 'New Statement',
				 table: '',
				 where_selections: [{key: '', value: null, type: null}]
			};
			state.statements.value.push(new_statement);
		},
		ADD_WHERE: (state, index) => {
			state.statements.value[index].where_selections.push({key: '', value: null, type: null});
		},
		REMOVE_WHERE: (state, indexes) => {
			console.log('indexes: ', indexes);
			state.statements.value[indexes.index].where_selections.splice(indexes.whereindex, 1);
		},
		REMOVE_STATEMENT: (state, index) => {
			state.statements.value.splice(index, 1);
		},
		DONE_ADDING_STATEMENTS: (state) => {
			state.statements.done = true;
		},
		ADD_COLUMN: (state, index) => {
			state.statements.value[index].column_selections.value.push({column: null, name: null, type: null, use_source: false });
		},
		DONE_ADDING_COLUMNS: (state) => {
			state.statements.columnSelectionsDone = true;
			csv2sql.then(m => {
				let result = m.process_file(state.raw_csv, state.statements);
				state.errors = result.value;
			}).catch(console.error)
		},
		REMOVE_COLUMN: (state, indexes) => {
			state.statements.value[indexes.index].column_selections.value.splice(indexes.index2, 1);
		},
		SOLVE_ERROR: (state, index) => {

			let newValue = state.errors[index].error_text;
			let type = state.errors[index].type;
			csv2sql.then(m => {
				let errorCorrected = m.check_correction(newValue, type);
				if (errorCorrected)
				{
					state.error_solutions.push(state.errors[index]);
					state.errors.splice(index, 1);
				}
			})


			console.log(state.error_solutions);
		},
		GENERATE_SQL: (state) =>
		{
			csv2sql
				.then(m => {
					let result = m.generate_file(state.raw_csv, state.statements, { value: state.error_solutions });

					var i = 0;
					for (i; i < result.length; i++) {
						var data = result[i];
						console.log("result:", data);
						var file_name = state.statements.value.find(function(element) {
							return element.id == i;
						}).name;
						console.log("file_name:", file_name);

						var blob = new Blob([data], {type: "application/sql"});
						var url = window.URL.createObjectURL(blob);
						state.downloads.push({ name: file_name + ".sql", url: url });
					}
				})
				.catch(console.error)
		}
	}
});

new Vue({
  store,
  render: h => h(App),
}).$mount('#app')
