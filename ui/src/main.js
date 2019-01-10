import Vue from 'vue'
import Vuex from 'vuex'
import App from './App.vue'

Vue.config.productionTip = false
Vue.use(Vuex);
var csv2sql = import('./csv2sql/csv2sql');

var default_state = function() {
	return {
		columns: [],
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
	}
};

//var state_stack = [];

var store = new Vuex.Store({
	state: Object.assign({}, default_state()),
	mutations: {
	LOAD_CSV: (state, data) => {
	var result = csv2sql._v.get_columns(data);
		if (result) {
			state.columns = result.columns;
			state.raw_csv = data;
			state.loaded = true;
		}
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
	}
});

new Vue({
  store,
  render: h => h(App),
}).$mount('#app')
