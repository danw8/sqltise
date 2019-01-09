<template>
    <div class="column-selections">
        <div v-for="(column, index) in columns" :key="index">
            {{column}}
        </div>
    </div>
</template>

<script>
import { mapState, mapMutations } from 'vuex'

export default {
    name: 'ColumnSelections',
	methods: {
		...mapMutations([
        ]),
        // other methods
    },
    computed: {
		...mapState([
            "columns"
		]),
		// other properties
	},
}
</script>

<style scoped>
.column-selections{
    margin-top: 20px;
}
</style>