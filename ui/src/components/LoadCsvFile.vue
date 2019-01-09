<template>
	<form id="fileform" ref="fileform">
		<div>Load the csv file to use as the source</div>
		<input type="file" id="files" name="csvfile" accept="text/csv" v-on:change="fileSelected"/>
	</form>
</template>

<script>
export default {
	name: 'LoadCsvFile',
	methods: {
		fileSelected : function (e) {
			var files = e.target.files;
			if (files.length > 1) {
				// validation should fail
				alert("Too many files");
				this.$refs.fileform.reset();
				return;
			}

			if (files.length < 1) {
				// validation should fail
				alert("Need at least 1 file");
				this.$refs.fileform.reset();
				return;
			}

			var file = files[0];
			if (file.type !== 'text/csv'){
				// validation should fail
				alert("Not a csv file");
				this.$refs.fileform.reset();
				return;
			}

			var reader = new FileReader();
			reader.readAsText(file);
			reader.onload = () => {
				alert(reader.result);
				// this seems to work except I'm not listening on the parent yet
				this.$root.$emit('loaded', file);
			}
		}
	}
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.load-file {
	width: 200px;
	height: 50px;
	background-color: black;
}
</style>
