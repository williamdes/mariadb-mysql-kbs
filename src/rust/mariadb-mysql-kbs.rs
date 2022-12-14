pub mod cleaner;
pub mod data;
pub mod extract;
pub mod find_missing_data;
pub mod mariadb;
pub mod mysql;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "mariadb-mysql-kbs")]
#[command(
    author,
    about = "MariaDB MySQL KBs",
    long_about = "MariaDB MySQL KBs program to extract data from sources"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(args_conflicts_with_subcommands = true, about = "Extract the data")]
    Extract {
        #[arg(
            long,
            require_equals = true,
            num_args = 1,
            value_name = "dataset",
            default_value_t = ExtractCommands::All,
            default_missing_value = "all",
            help = "The dataset to import",
            value_enum
        )]
        dataset: ExtractCommands,
    },
    #[command(args_conflicts_with_subcommands = true, about = "Find missing data")]
    FindMissingData {
        #[arg(
            long,
            require_equals = true,
            num_args = 1,
            value_name = "source",
            default_value_t = ExtractCommands::All,
            default_missing_value = "all",
            help = "The source to check",
            value_enum
        )]
        source: ExtractCommands,
    },
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum ExtractCommands {
    All,
    Mysql,
    Mariadb,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Extract { dataset } => match dataset {
            ExtractCommands::All => {
                extract::extract(extract::ExtractionPreference::All);
            }
            ExtractCommands::Mysql => {
                extract::extract(extract::ExtractionPreference::MySQL);
            }
            ExtractCommands::Mariadb => {
                extract::extract(extract::ExtractionPreference::MariaDB);
            }
        },
        Commands::FindMissingData { source } => match source {
            ExtractCommands::All => {
                find_missing_data::run(extract::ExtractionPreference::All);
            }
            ExtractCommands::Mysql => {
                find_missing_data::run(extract::ExtractionPreference::MySQL);
            }
            ExtractCommands::Mariadb => {
                find_missing_data::run(extract::ExtractionPreference::MariaDB);
            }
        },
    }
}
