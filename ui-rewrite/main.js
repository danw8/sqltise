// This is the .csv file data that was chosen.
var file_data;

// This function resets the app back to its default state.
var reset_app =  function() {
	file_data = undefined;
	statement_selections.data = {
		statements: [],
		valid: false,
		one_file: true,
	};

	load_file.render();
}

/* This object lets the user select the statement option they wish to use */
var statement_selections = {
	data: {
		statements: [],
		valid: false,
		one_file: true,
	},
	check_valid: function(){
		var is_valid = true;
		if (this.data.statements.length === 0) {
			is_valid = false;
		}
		if (!this.data.statements.every((s) => !!s.name && !!s.table && !!s.type)) {
			is_valid = false;
		}
		this.data.valid = is_valid;
		var done_button = document.getElementById('selection-done');
		done_button.disabled = !this.data.valid;
	},
	change_selection: function(index, element) {
		this.data.statements[index].type = element.value;
		this.check_valid();
	},
	change_name: function(index, element) {
		this.data.statements[index].name = element.value;
		this.check_valid();
	},
	change_table_name: function(index, element) {
		this.data.statements[index].table = element.value;
		this.check_valid();
	},
	toggle_one_file: function() {
		this.data.one_file = !this.data.one_file;
		this.check_valid();
	},
	remove: function(index) {
		this.data.statements.splice(index, 1);
		this.check_valid();
		this.render();
	},
	add: function() {
		this.data.statements.push(
			{
				type: "Insert",
				name: '',
				table: '',
			}
		);
		this.check_valid();
		this.render();
	},
	done: function() {
		alert('done');
	},
	template: function() {
		var title = `<h3>Statement Selection</h3>`
		var reset = `<button class="large-button" onclick="reset_app()">Reset</button><br/>`;
		var warnings = ``;
		console.log(this);
		if (this.data.statements.length === 0) {
			warnings = `<h5>You must select at least one statement</h5>`;
		}

		var table = ``;
		if (this.data.statements.length > 0){
			var table_header = `
				<tr>
					<th>Type</th>
					<th>Name</th>
					<th>Table</th>
					<th>Delete</th>
				</tr>`;
			var table_body = ``;
			//for (const [i, statement] of this.data.statements) {
			for(var i = 0; i < this.data.statements.length; i++) {
				var statement = this.data.statements[i];
				var select = `
					<select onchange="statement_selections.change_selection(${i},this)">
						<option ${statement.type === 'Insert'? 'selected': ''}>Insert</option>
						<option ${statement.type === 'Update'? 'selected': ''}>Update</option>
						<option ${statement.type === 'Custom'? 'selected': ''}>Custom</option>
					</select>
				`;

				var name = `<input type="text" 
					onkeyup="statement_selections.change_name(${i}, this)"
					onchange="statement_selections.change_name(${i}, this)"
					value="${statement.name}"/>`;
				var table_name = `<input type="text"
					onkeyup="statement_selections.change_table_name(${i}, this)"
					onchange="statement_selections.change_table_name(${i}, this)"
					value="${statement.table}"/>`;
				var del_button = `<button class="delete-button" onclick="statement_selections.remove(${i})">X</button>`;
				table_body += `
					<tr>
						<td>${select}</td>
						<td>${name}</td>
						<td>${table_name}</td>
						<td>${del_button}</td>
					</tr>
				`
			}
			table = `
				<table>
					${table_header}
					${table_body}
				</table>
			`;
		}
		var options = `
			<label for="single-file-option">Generate all statements into a single file</label>
			<input id="single-file-option" type="checkbox" 
			${this.data.one_file ? `checked` : ``} 
			onchange="statement_selections.toggle_one_file()">`;
		var controls = `
			<div class="controls">
				<button onclick="statement_selections.add()">Add Statement</button>
				<button id="selection-done"onclick="statement_selections.done()" ${this.data.valid ? `` : `disabled`}>Done</button>
			</div>
		`;
		var help = `<p>Choose what kind of statements you want sqltise to create.</p>`
		return `
			${reset}
			${title}
			${warnings}
			${options}
			${table}
			${controls}
			${help}
		`;
	},
	render: function() {
		var app = document.getElementById("app");
		app.innerHTML = this.template();
	}
}

/* This object loads the .csv file into memory */
var load_file = {
	data: {
		invalid: false,
	},
	load: function() {
		var file_input = document.createElement('input');
		file_input.type = 'file';

		file_input.onchange = (e) => {
			var files = e.target.files;
			if (files.length > 1 || files.length < 1) {
				this.data.invalid = true;
				this.render();
				return;
			}

			var file = files[0];
			var types = ['text/csv', 'application/vnd.ms-excel'];
			if (!types.includes(file.type) && !file.name.endsWith('.csv')){
				this.data.invalid = true;
				this.render();
				return;
			}

			var reader = new FileReader();
			reader.readAsText(file);
			reader.onload = () => {
				file_data = reader.result;
				// Start the next step.
				statement_selections.render();
			}
		}

		file_input.click();
	},
	template: function() {
		var explaination = `<p class="skinny">This first step of Sqltise is to choose a '.csv' file. This file should be prepared in excel or another program that creates '.csv' files</p>`
		var button = `<button class="large-button" onclick="load_file.load()">Load File</button>`
		var invalid_text = ``;
		if (this.data.invalid) {
			invalid_text = `<p>You must select 1 and only 1 file. The file must be a csv file.</p>`;
		}
		
		return `
			${invalid_text}
			${button}
			${explaination}
		`;
	},
	render: function() {
		var app = document.getElementById("app");
		app.innerHTML = this.template();
	}
}

load_file.render();