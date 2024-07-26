use clap::{Command, Arg};
pub mod common;

fn cli() -> Command {

    Command::new("caf")
        .about("caf - cloud adoption framework")
        .version("1.0")
        .author("deixei <deixei@deixei.com>")
        .subcommand(
            Command::new("init")
                .about("Initializes a change management project")
                .arg(Arg::new("name")
                    .long("name")
                    .short('n')
                    .required(true))
                .arg(Arg::new("template")
                    .long("template")
                    .short('t')
                    .required(true)),
        )
        .subcommand(
            Command::new("run")
                .about("Runs a playbook")
                .arg(Arg::new("name")
                    .long("name")
                    .short('n')
                    .default_value("playbook")
                    .required(false))
                .arg(Arg::new("path")
                    .long("path")
                    .short('p')
                    .default_value("")
                    .required(false))
                .arg(Arg::new("verbose")
                    .long("verbose")
                    .short('v')
                    .help("Sets the level of verbosity")
                    .default_value("")
                    .required(false))
                .arg(Arg::new("arguments")
                    .long("arguments")
                    .short('a')
                    .default_value("STAGE=dev")
                    .required(false)),
        )
        .subcommand(
            Command::new("build")
                .about("Builds chgops")
                .arg(Arg::new("debug")
                    .long("debug")
                    .short('d')
                    .required(true))
                .arg(Arg::new("change_id")
                    .long("change_id")
                    .short('c')
                    .required(true)),
        )
        .subcommand(
            Command::new("test")
                .about("Tests chgops")
                .arg(Arg::new("scope")
                    .long("scope")
                    .short('s')
                    .required(true)),
        )
        .subcommand(
            Command::new("publish")
                .about("Publishes chgops")
                .arg(Arg::new("ado_pack")
                    .long("ado_pack")
                    .required(true))
                .arg(Arg::new("package")
                    .long("package")
                    .short('p')
                    .required(true)),
        )
        .subcommand(
            Command::new("download")
                .about("Downloads chgops")
                .arg(Arg::new("name")
                    .long("name")
                    .short('n')
                    .required(true))
                .arg(Arg::new("version")
                    .long("version")
                    .short('v')
                    .required(true)),
        )
}

fn main() {
    let matches: clap::ArgMatches = cli().get_matches();

    print_banner_yellow!("--------------------------------------------------------");
    print_banner_yellow!("CAF - Cloud Adoption Framework - deixei - Marcio Parente");
    print_banner_yellow!("--------------------------------------------------------");

    match matches.subcommand() {
        Some(("init", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").expect("required");
            let template = sub_matches.get_one::<String>("template").expect("required");

            print_info!("Initializing with name: {}, template: {}", name, template);


        }
        Some(("run", sub_matches)) => {
            
            let playbook_name = sub_matches.get_one::<String>("name").expect("required");
            let workspace_path = sub_matches.get_one::<String>("path").expect("required");
            let verbose = sub_matches.get_one::<String>("verbose").expect("required");
            let arguments = sub_matches.get_one::<String>("arguments").expect("required");

            print_info!("Running playbook: {}, verbose: {}, arguments: {}, workspace: {}", playbook_name, verbose, arguments, workspace_path);

        }
        Some(("build", sub_matches)) => {
            print_info!(
                "Building with debug: {}, change_id: {}",
                sub_matches.get_one::<String>("debug").expect("required"),
                sub_matches.get_one::<String>("change_id").expect("required")
            );
        }
        Some(("test", sub_matches)) => {
            let scope = sub_matches.get_one::<String>("scope").expect("required");
            print_info!("Testing with scope: {}", scope);
            
        }
        Some(("publish", sub_matches)) => {
            print_info!(
                "Publishing with ado_pack: {}, package: {}",
                sub_matches.get_one::<String>("ado_pack").expect("required"),
                sub_matches.get_one::<String>("package").expect("required")
            );
        }
        Some(("download", sub_matches)) => {
            print_info!(
                "Downloading with name: {}, version: {}",
                sub_matches.get_one::<String>("name").expect("required"),
                sub_matches.get_one::<String>("version").expect("required")
            );
        }
        Some((ext, sub_matches)) => {
            let args = sub_matches
                .get_many::<String>("")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
                print_info!("Calling out to {ext:?} with {args:?}");
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!
    }
}