use clap::{App, Arg}; // tell Rust you will use these two structs in clap
use lib::{
    run_add, run_find, run_grep, run_remove, run_tr, AddConfig, FindConfig, GrepConfig,
    RemoveConfig, TrConfig,
}; // tell Rust you will use these many things from our "lib" module

fn main() {
    // Define command-line interface
    let matches = App::new("rust")
        .version("0.1.0")
        .author("Yonas Mekonnen, Hongyang Lin, Spencer Chan")
        .about("Emulate basic Linux commands")
        .subcommand(
            App::new("find")
                .arg(
                    Arg::from("-p , --patterns=<patterns> 'List of file patterns to find.'")
                        .takes_value(true)
                        .required(true)
                        .multiple_values(true), // this argument can takes multiple values
                )
                .arg(
                    Arg::from("-o, --output=<output> 'Write results to output file instead of stdout.'")
                        .takes_value(true) // argument if true or flag if false.
                        .required(false), // this is an optional argument
                )
                .arg(
                    Arg::from("-d, --dirs=<dirs> 'Set of directories'")
                        .takes_value(true)
                        .required(true)
                        .multiple_values(true),
                )
                .arg(
                    Arg::from("-s, --size=<size> 'Minimum size in bytes that a matched file needs to have to be reported'")
                        .takes_value(true)
                        .required(false),
                )
                .arg(
                    Arg::from("-x, --exec=<cmd> 'The command to run'")
                    .takes_value(true)
                    .required(false)
                    .requires("replace")
                    .multiple_values(true)
                )
                .arg(
                    Arg::from("-r, --replace=<replace_str> 'Replace occurences of \'replace str\' with names of found files.")
                    .takes_value(true)
                    .required(false)
                    .requires("exec")
                )
                .arg(
                    Arg::from("-a, --all 'Whether to use all arguments'")
                    .takes_value(false)
                    .required(false)
                    .requires_all(&["replace", "exec"])
                )
                // could thread here! just spawn 1 command per found item...
        )
        .subcommand(
            App::new("add")
                .arg(
                    Arg::from("-f, --files=<files> 'Name of the file that's to be added")
                        .takes_value(true)
                        .required(true)
                        .multiple_values(true),
                )
                .arg(
                    Arg::from("-d, --dirs=<dirs> 'Set of directories'")
                        .takes_value(true)
                        .required(true)
                        .multiple_values(true),
                )
        )
        .subcommand(
            App::new("remove")
                .arg(
                    Arg::from("-f, --files=<files> 'Name of the file that's to be removed'")
                        .takes_value(true)
                        .required(true)
                        .multiple_values(true),
                )
                .arg(
                    Arg::from("-d, --dirs=<dirs> 'Set of directories'")
                        .takes_value(true)
                        .required(true)
                        .multiple_values(true),
                )
        )
        .subcommand(
            // tr++
            App::new("tr")
                .arg(
                    Arg::from("-p, --path=<path> 'Path of file to be modified'")
                        .takes_value(true)
                        .required(true)
                )
                .arg(
                    Arg::from("-f, --file=<file> 'Name of the file that's to be modified'")
                        .takes_value(true)
                        .required(true)
                )
                .arg(
                    Arg::from("-d, --delete=<delete> 'String to be deleted in the specified file")
                        .takes_value(true)
                        .required(false)
                        .multiple_values(true),
                )
                .arg(
                    Arg::from("-r, --replace=<replace> 'Replace repeated string listed in the file with single occurrence")
                        .takes_value(true)
                        .required(false)
                        .multiple_values(true),
                )
                .arg(
                 Arg::from("-s, --simulate 'Print the contents of the file with replacement, without overwriting it'")
                     .takes_value(false)
                     .required(false)
                )
        )
        .subcommand(
            App::new("grep")
                .arg(
                    Arg::from("-p, --patterns=<pattern> 'pattern to be searched'")
                        .takes_value(true)
                        .multiple_values(true)
                        .required(true)
                )
                .arg(
                    Arg::from("-f, --filenames=<filename> 'name of file to be searched'")
                        .takes_value(true)
                        .multiple_values(true)
                        .required(true)
                )
        )
        .get_matches();
    // .get_matches_from(vec!["rust", "find", "--patterns=.*/.rs", "--output=./tests.out", "--dirs=./"]);

    if let Some(sub_m) = matches.subcommand_matches("find") {
        let args = FindConfig::from_args(sub_m);

        if let Err(err) = run_find(&args) {
            //Error handling here!
            panic!("{}", err)
        }
    } else if let Some(sub_m) = matches.subcommand_matches("add") {
        let args = AddConfig::from_args(sub_m);

        if let Err(err) = run_add(&args) {
            panic!("{}", err)
        }
    } else if let Some(sub_m) = matches.subcommand_matches("remove") {
        let args = RemoveConfig::from_args(sub_m);

        if let Err(err) = run_remove(&args) {
            panic!("{}", err)
        }
    } else if let Some(sub_m) = matches.subcommand_matches("tr") {
        let args = TrConfig::from_args(sub_m);

        if let Err(err) = run_tr(&args) {
            panic!("{}", err)
        }
    } else if let Some(sub_m) = matches.subcommand_matches("grep") {
        let args = GrepConfig::from_args(sub_m);

        if let Err(err) = run_grep(&args) {
            panic!("{}", err)
        }
    }
}
