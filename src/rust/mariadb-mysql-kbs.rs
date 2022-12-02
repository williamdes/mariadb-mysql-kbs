pub mod cleaner;
pub mod data;
pub mod extract;
pub mod mariadb;
pub mod mysql;

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
