/*!
One-line description.

More detailed description, with

# Example

*/

use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;
use terminal_travel::config::{get_app_config_from, get_app_config_path, get_stage};
use terminal_travel::config::{AppConfig, APP_CONFIG_NAME};
use terminal_travel::itinerary::display::{display_itinerary, DisplayFormat};
use terminal_travel::itinerary::io::from_reader;
use tracing::{debug, info};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, StructOpt)]
#[structopt(name = APP_CONFIG_NAME)]
struct CommandLine {
    /// The level of logging to perform; from off to trace
    #[structopt(long, short = "v", parse(from_occurrences))]
    verbose: i8,

    #[structopt(short, long, parse(from_os_str))]
    /// Use an alternate configuration from the named file
    config_file: Option<PathBuf>,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    /// Create a new itinerary file
    New {
        #[structopt(short, long)]
        /// Use interactive mode
        interactive: bool,

        #[structopt(name = "FILE", parse(from_os_str))]
        /// Itinerary file path
        file: PathBuf,
    },
    /// Update an existing itinerary file
    Edit {
        #[structopt(short, long)]
        /// Use interactive mode
        interactive: bool,

        #[structopt(name = "FILE", parse(from_os_str))]
        /// Itinerary file path
        file: PathBuf,
    },
    /// Display an existing itinerary file
    Display {
        #[structopt(long)]
        update_flights: bool,

        #[structopt(name = "FILE", parse(from_os_str))]
        /// Itinerary file path
        file: PathBuf,
    },
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// async fn check_flight_schedules() -> Result<(), Box<dyn Error>> {
//     let mut config = get_app_config()?;
//     println!(
//         "{:?}",
//         fetch_flight_schedule(
//             &mut config,
//             &flight_schedule_request(
//                 AirCarrierCode::from_str("AA").unwrap(),
//                 1471,
//                 Date::from_ymd(2022, 6, 24),
//                 None
//             )?
//         )
//         .await?
//     );
//
//     Ok(())
// }

async fn cmd_new_itinerary(
    _file: PathBuf,
    _interactive: bool,
    _app_config: AppConfig,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}

async fn cmd_edit_itinerary(
    _file: PathBuf,
    _interactive: bool,
    _app_config: AppConfig,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}

async fn cmd_display_itinerary(
    file: PathBuf,
    _update_flights: bool,
    _app_config: AppConfig,
) -> Result<(), Box<dyn Error>> {
    if !file.is_file() {
        eprintln!("Error: file '{:?}' does not exist", file);
    }
    let file = File::open(&file)?;
    let itinerary = from_reader(file)?;
    display_itinerary(&itinerary, DisplayFormat::default())?;
    Ok(())
}

fn init_tracing(level: i8) {
    use terminal_travel::config::Stage;
    use tracing_subscriber::filter::LevelFilter;
    use tracing_subscriber::EnvFilter;

    let env_filter = EnvFilter::from_default_env().add_directive(
        match level {
            0 => LevelFilter::OFF,
            1 => LevelFilter::ERROR,
            2 => LevelFilter::WARN,
            3 => LevelFilter::INFO,
            4 => LevelFilter::DEBUG,
            _ => LevelFilter::TRACE,
        }
        .into(),
    );

    let file_appender = tracing_appender::rolling::daily(".", &format!("{}.log", APP_CONFIG_NAME));

    match get_stage() {
        Stage::Development => tracing_subscriber::fmt()
            .with_env_filter(env_filter)
            .with_writer(file_appender)
            .with_file(true)
            .with_line_number(true)
            .with_thread_ids(true)
            .pretty()
            .init(),
        Stage::Test => tracing_subscriber::fmt()
            .with_env_filter(env_filter)
            .with_writer(file_appender)
            .with_file(true)
            .with_line_number(true)
            .with_thread_ids(true)
            .init(),
        Stage::Production => tracing_subscriber::fmt()
            .with_env_filter(env_filter)
            .with_writer(file_appender)
            .with_thread_ids(true)
            .compact()
            .init(),
    }

    info!("Logging initialized");
}

// ------------------------------------------------------------------------------------------------
// Entry Point
// ------------------------------------------------------------------------------------------------

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    human_panic::setup_panic!();
    
    let cmd_line = CommandLine::from_args();
    debug!("{:?}", cmd_line);

    init_tracing(cmd_line.verbose);

    let config_file = match cmd_line.config_file {
        Some(f) => f.clone(),
        None => get_app_config_path(),
    };

    let app_config = get_app_config_from(&config_file)?;

    match cmd_line.cmd {
        Command::New { interactive, file } => {
            cmd_new_itinerary(file, interactive, app_config).await?
        }
        Command::Edit { interactive, file } => {
            cmd_edit_itinerary(file, interactive, app_config).await?
        }
        Command::Display {
            update_flights,
            file,
        } => cmd_display_itinerary(file, update_flights, app_config).await?,
    }

    Ok(())
}
