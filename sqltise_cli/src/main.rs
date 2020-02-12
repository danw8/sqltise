use clap::{arg_enum, value_t, App, AppSettings, Arg, SubCommand};
use std::path::Path;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

arg_enum! {
	#[derive(Debug)]
	pub enum Selections {
		Statement,
		SourceFile,
		Project,
	}
}

fn main() {
	let matches = App::new("sqltise")
		.version(VERSION)
		.setting(AppSettings::SubcommandRequiredElseHelp)
		.about("Convert and cleanse csv data in sql scripts")
		.arg(
			Arg::with_name("project")
				.short("p")
				.long("project")
				.help("The name of the project")
				.takes_value(true),
		)
		.subcommand(
			SubCommand::with_name("add")
				.about("adds selections to the project")
                .arg(Arg::with_name("selection")
                     .help("the type to add")
                     .possible_values(&["statement", "sourcefile", "project"])
                     .index(1)
                     .required(true))
		)
		.subcommand(
			SubCommand::with_name("remove")
				.about("remove selections from the project")
                .arg(Arg::with_name("selection")
                     .help("the type to remove")
                     .possible_values(&["statement", "sourcefile", "project"])
                     .index(1)
                     .required(true))
		)
        .subcommand(
            SubCommand::with_name("list")
                .about("list the current selections from the project")
                .arg(Arg::with_name("selection")
                     .help("the type to list")
                     .possible_values(&["statement", "sourcefile", "all"])
                     .index(1)
                     .required(true))
        )
		.get_matches();

	let project_name = matches.value_of("project").unwrap_or("test");
	dbg!(project_name);

	if let Some(matches) = matches.subcommand_matches("add") {
        let selection = value_t!(matches, "selection",  Selections).expect("Must supply a variant");
        let x = format!("add {}", selection);
		dbg!(x);
	}
	if let Some(matches) = matches.subcommand_matches("remove") {
		dbg!("remove statements");
	}
}
