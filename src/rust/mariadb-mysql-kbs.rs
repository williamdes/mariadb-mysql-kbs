pub mod extract;

fn main() {
    let cmd = clap::Command::new("mariadb-mysql-kbs")
        .bin_name("mariadb-mysql-kbs")
        .subcommand_required(true)
        .subcommand(clap::command!("extract"));
    let matches = cmd.get_matches();
    match matches.subcommand() {
        Some(("extract", _)) => extract::extract(),
        _ => unreachable!("clap should ensure we don't get here"),
    };
}
