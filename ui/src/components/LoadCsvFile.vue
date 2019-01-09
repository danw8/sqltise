<template>
	<form id="fileform" ref="fileform">
		<div class="file-upload">
			<input type="file" id="files" ref="files" name="csvfile" class="csv-upload" accept="text/csv" v-on:change="fileSelected"/>
			<div class="csv-upload-label"  v-on:click="selectfile"><i class="fa fa-file-upload upload-icon"></i><span ref="filelabel">CHOOSE A FILE</span></div>
		</div>
		<div>Load the csv file to use as the source</div>
	</form>
</template>

<script>
import { mapMutations } from 'vuex';

export default {
	name: 'LoadCsvFile',
	methods: {
		...mapMutations([
			'LOAD_CSV',
		]),
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
			if (file.type !== 'text/csv') {
				if (file.type !== 'application/vnd.ms-excel') {
					// validation should fail
					alert("Not a csv file");
					this.$refs.fileform.reset();
					return;
				}

				if (!file.name.endsWith(".csv")) {
					// validation should fail
					alert("Not a csv file");
					this.$refs.fileform.reset();
					return;
				}
				
			}

			this.$refs.filelabel.textContent = file.name;

			var reader = new FileReader();
			reader.readAsText(file);
			reader.onload = () => {
				this.LOAD_CSV(reader.result); 
			}
		},
		selectfile: function () {
			this.$refs.files.click();
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

.file-upload {
	position: relative;
	margin: 10px 0px;
}

.csv-upload-label {
	position: absolute;
	top: 0px;
	left: 0px;
	width: 240px;
	height: 60px;
	z-index: 2;
	color: white;
	font-size: 1.25em;
	background-color: #277554;
	display: flex;
	justify-content: center;
	align-items: center;
	cursor: pointer;
}

.csv-upload-label:hover {
	background-color: #499273;
}

.csv-upload {
	position: relative;
	text-align: right;
	-moz-opacity: 0;
	filter:alpha(opacity:0);
	opacity: 0;
	z-index: 1;
	width: 240px;
	height: 60px;
	cursor: pointer;
}

.upload-icon{
	font-size: 30px;
	padding-right: 10px;
}
</style>
