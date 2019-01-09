import Vue from 'vue'
import Vuex from 'vuex'
import App from './App.vue'

Vue.config.productionTip = false
Vue.use(Vuex);
var csv2sql = import('./csv2sql/csv2sql');

var default_state = function() {
  return {
    columns: {},
    raw_csv: null,
    loaded: false,
    statements: {
      done: false,
      value: []
    },
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
      state.statements.value.push({ type: 'Insert', name: 'New Statement', table: '' });
    },
    REMOVE_STATEMENT: (state, index) => {
      state.statements.value.splice(index, 1);
    },
    DONE_ADDING_STATEMENTS: (state) => {
      state.statements.done = true;
    }
	}
});

new Vue({
  store,
  render: h => h(App),
}).$mount('#app')
