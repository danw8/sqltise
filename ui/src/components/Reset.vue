<template>
    <button class="reset-button" v-on:click="reset">RESET</button>
</template>

<script>
import { mapState, mapMutations } from 'vuex'

export default {
    name: 'Reset',
	methods: {
		...mapMutations([
            "RESET"
        ]),
        reset: function() {
            this.RESET();
        }
    },
    computed: {
		...mapState([
		]),
		// other properties
	},
}
</script>

<style scoped>
.reset-button{
    background-color: #277554;
    border: none;
    color: white;
    font-size: 1.25em;
    padding: 8px;
    height: 60px;
    width: 240px;
    cursor: pointer;
}

.reset-button:hover {
    background-color: #499273;
}
</style>