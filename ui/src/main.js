import Vue from 'vue'
import Vuex from 'vuex'
import App from './App.vue'

Vue.config.productionTip = false
Vue.use(Vuex);
var csv2sql = import('./csv2sql/csv2sql');

var default_state = function() {
	return {
		columns: [{name:"StateCode",index:0}],
		column_selections: {
			value: [],
			done: false,
		},
		raw_csv: null,
		loaded: false,
		statements: {
			done: false,
			nextid: 0,
			value: []
		},
		debug: false,
		error_solutions: [],
		errors: [],
	}
};

//var state_stack = [];

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
		state.statements.value.push({ id: id, type: 'Insert', name: 'New Statement', table: '' });
	},
	REMOVE_STATEMENT: (state, index) => {
		state.statements.value.splice(index, 1);
	},
	DONE_ADDING_STATEMENTS: (state) => {
		state.statements.done = true;
	},
	ADD_COLUMN: (state) => {
		state.column_selections.value.push({column: null, statement_id: null, name: null, type: null, use_source: false });
	},
	DONE_ADDING_COLUMNS: (state) => {
		state.column_selections.done = true;
		csv2sql.then(m => {
			let result = m.process_file(state.raw_csv, state.column_selections);
			state.errors = result.value;
		}).catch(console.error)
	},
	REMOVE_COLUMN: (state, index) => {
		state.column_selections.value.splice(index, 1);
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
				console.log("this should call the generate sql endpoint in rust");
				let result = m.generate_file(state.raw_csv, state.statements, state.column_selections, state.error_solutions);
				console.log("What we got",result);
				if (result.length)
				{
					console.log('Result', result);
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
