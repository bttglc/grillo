use clap::{Arg, Command};

pub enum CliCommand {
    Add { description: String },
    List,
    Delete { ids: Vec<u64> },
    Done { ids: Vec<u64> },
    Help,
}

pub fn parse_args() -> CliCommand {
    let matches = Command::new("grillo")
        .version("0.1.0")
        .about("A task management tool")
        .subcommand_required(false)
        .arg_required_else_help(false)
        .subcommand(
            Command::new("add")
                .about("Add a new task")
                .arg(
                    Arg::new("description")
                        .help("Task description")
                        .required(true)
                        .index(1)
                )
        )
        .subcommand(
            Command::new("del")
                .about("Delete tasks")
                .arg(
                    Arg::new("ids")
                        .help("Task IDs to delete")
                        .required(false)
                        .num_args(1..)
                        .value_parser(clap::value_parser!(u64))
                )
        )
        .subcommand(
            Command::new("done")
                .about("Mark tasks as done")
                .arg(
                    Arg::new("ids")
                        .help("Task IDs to mark as done")
                        .required(false)
                        .num_args(1..)
                        .value_parser(clap::value_parser!(u64))
                )
        )
        .subcommand(Command::new("ls").about("List all tasks"))
        .get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let description = sub_matches
                .get_one::<String>("description")
                .unwrap()
                .to_string();
            CliCommand::Add { description }
        }
        Some(("del", sub_matches)) => {
            let ids = sub_matches
                .get_many::<u64>("ids")
                .map(|vals| vals.copied().collect())
                .unwrap_or_else(Vec::new);
            CliCommand::Delete { ids }
        }
        Some(("done", sub_matches)) => {
            let ids = sub_matches
                .get_many::<u64>("ids")
                .map(|vals| vals.copied().collect())
                .unwrap_or_else(Vec::new);
            CliCommand::Done { ids }
        }
        Some(("ls", _)) => CliCommand::List,
        _ => CliCommand::Help,
    }
}
