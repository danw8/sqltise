<template>
	<form id="fileform" ref="fileform">
		<div>Load the csv file to use as the source</div>
		<div class="file-upload">
			<input type="file" id="files" ref="files" name="csvfile" class="csv-upload" accept="text/csv" v-on:change="fileSelected"/>
			<div class="csv-upload-label"  v-on:click="selectfile"><i class="fa fa-file-upload upload-icon"></i><span ref="filelabel">Choose a file</span></div>
		</div>
	</form>
</template>

<script>
export default {
	name: 'LoadCsvFile',
	methods: {
		fileSelected : function (e) {
			var files = e.target.files;
			// eslint-disable-next-line
			console.log(files);
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
			// eslint-disable-next-line
			console.log(file.type);
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
				alert(reader.result);
				// this seems to work except I'm not listening on the parent yet
				this.$root.$emit('loaded', file);
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
