<template>
    <div class="table-wrapper">   
        <table class="errors-table">
            <thead>
                <tr>
                    <th>Invalid Data</th>
                    <th>Type</th>
                    <th>Count</th>      
                    <th>Action</th>
                </tr>
            </thead>
            <tbody>
                <tr v-for="(error, index) in errors" :key="index">
                    <td><input type="text" v-model="error.error_text" /></td>
                    <td>{{error.type}}</td>
                    <td>{{error.rows.length}}</td>
                    <td><button class="validate-button" v-on:click="solve(index)">Validate</button></td>
                </tr>
            </tbody>
        </table>
    </div>
</template>

<script>
import { mapState, mapMutations } from 'vuex';

export default{
    name: 'SolveErrors',
	methods: {
		...mapMutations([
			'SOLVE_ERROR',
        ]),
         solve: function (index) {
            this.SOLVE_ERROR(index);
        },
    },
    computed: {
		...mapState([
            "errors",
            "columns"
        ]),
		// other properties
    },
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.table-wrapper {
    width: 50%;
    margin-top: 16px;
    border:1px solid red;
    padding: 8px;
}

.errors-table {
    width: 100%;
    text-align: left;
    border-collapse: collapse;
}

.errors-table > thead {
     border-bottom: 1px solid grey;
 }


th,td {
    padding-left: 8px;
}

 tr > td {
    
    padding-top: 8px;
    padding-bottom: 8px;
 }

 .validate-button {
   background-color: #277554;
	border: none;
	color: white;
	padding: 8px;
    cursor: pointer;
 }

 .validate-button:hover {
	background-color: #499273;
}

</style>