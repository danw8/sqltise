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
		debug: true,
		errors: [
			{statement_id: 1,
			column_id: 1,
			type:'VARCHAR',
			error_text:'some failed text',
			rows:[1,2,3,4]
		}
		],
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
		state.column_selections.value.push({column: null, statement_id: null, name: null, type: null, usesource: false });
	},
	DONE_ADDING_COLUMNS: (state) => {
		state.column_selections.done = true;
	},
	REMOVE_COLUMN: (state, index) => {
		state.column_selections.value.splice(index, 1);
	},
	SOLVE_ERROR: (state, index) => {
		console.log('Hi ' + state.errors[index].error_text)
		let result = 
		csv2sql
			.then(m => {
				let result = m.solve_error(state.error[index])
				if (result === "SUCCESS")
				{
					state.errors.splice(index, 1)
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
