use clap::Command;

mod day1;

fn main() {
    let cmd = Command::new("advent-of-code-2023")
        .bin_name("advent-of-code-2023")
        .subcommand_required(true)
        .subcommand(day1::command());
    let matches = cmd.get_matches();
    match matches.subcommand() {
        Some(("day1", matches)) => day1::run(matches),
        _ => panic!("Sub-command is not yet implemented"),
    };
}
