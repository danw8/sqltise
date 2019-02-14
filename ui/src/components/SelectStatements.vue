<template>
    <div class="statement-selections">
        <h3>Statements</h3>
        <div v-if="statements.value.length === 0">
            <h5>There are no statments</h5>
        </div>
        <div class="statement-header" v-if="statements.value.length > 0">
            <div>Type</div>
            <div>Name</div>
            <div>Table</div>
            <div>Delete</div>
        </div>
        <div class="statement" v-for="(statement, index) in statements.value" :key="index">
            <select v-model="statement.type" class="statement-select">
                <option>Insert</option>
                <option>Update</option>
            </select>
            <input v-model="statement.name" class="statement-input"/>
            <input v-model="statement.table" class="statement-input"/>
            <button class="remove-button" v-on:click="remove(index)"><i class="fa fa-trash"></i></button>
        </div>
        <input type="checkbox" id="single-file" :checked="onefile" v-on:change="check_one_file">
        <label for="single-file">Generate all statements to a single file</label>
        <div class="action-buttons">
        <button class="statements-button" v-on:click="add">Add Statement<i class="fa fa-plus add-icon"></i></button>
        <button class="statements-button" v-on:click="done" v-if="statements_complete(statements.value)">Done<i class="fa fa-check add-icon"></i></button>
        </div>
    </div>
</template>

<script>
import { mapState, mapMutations } from 'vuex'

export default {
    name: 'SelectStatements',
	methods: {
		...mapMutations([
            "ADD_STATEMENT",
            "REMOVE_STATEMENT",
            "DONE_ADDING_STATEMENTS",
            "CHANGE_ONE_FILE_SELECTION",
        ]),
        add: function () {
            this.ADD_STATEMENT();
        },
        done: function () {
            this.DONE_ADDING_STATEMENTS();
        },
        remove: function(index) {
            this.REMOVE_STATEMENT(index);
        },
        statements_complete: function(statements) {
            if (!statements || statements.length === 0){
                return false;
            }
            return statements.every((s) => !!s.name && !!s.table && !!s.type);
        },
        check_one_file: function() {
            this.CHANGE_ONE_FILE_SELECTION(!this.onefile);
        }
        // other methods
    },
    computed: {
		...mapState([
            "statements",
            "onefile",
        ]),
		// other properties
	},
}
</script>

<style scoped>
.statement-selections{
    margin-top: 20px;
}

.statements-button{
    background-color: #277554;
    border: none;
    color: white;
    padding: 8px;
    margin-right: 16px;
}

.statements-button:hover {
    background-color: #499273;
}

.add-icon{
    padding-left: 8px;
}

.statement-header {
    margin: 16px 0px;
    width: 800px;
    display: flex;
}

.statement-header div{
    margin-right: 16px;
    text-align: center;
    min-width: 225px;
    max-width: 225px;
    font-weight: 700;
}

.statement{
    margin: 16px 0px;
    display: flex;
}

.statement-input{
    margin-right: 16px;
    font-size: 16px;
    padding: 8px;
    min-width: 207px;
    max-width: 207px;
}

.statement-select{
    margin-right: 16px;
    padding: 10px;
    font-size: 16px;
    min-width: 225px;
    max-width: 225px;
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

.action-buttons{
    margin-top: 20px;
}
</style>