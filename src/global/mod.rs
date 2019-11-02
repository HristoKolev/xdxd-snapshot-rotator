pub mod app_config;
pub mod errors;
pub mod extensions;
pub mod sentry_backtrace;
pub mod custom_sentry_client;
pub mod error_handler;
pub mod logging;
pub mod prelude;

#[macro_use]
pub mod bash_shell;
pub mod do_try;
pub mod email;
pub mod email_report;
pub mod cli;
pub mod file_lock;

use std::path::{PathBuf, Path};
use chrono::{DateTime, Utc};
use lazy_static::lazy_static;

use self::prelude::*;
use self::app_config::{AppConfig, read_config};
use self::custom_sentry_client::CustomSentryClient;
use self::error_handler::handle_error;
use self::logging::*;
use self::cli::CliRunner;

static APP_CONFIG_FILE_NAME: &str = "app-config.json";
static LOG_FILE_NAME: &str = "log/log.txt";
static LOG_FILE_MAX_LENGTH: u64 = 1024000; // 10MB

/// The global object struct.
pub struct Global {
    pub app_config: AppConfig,
    pub sentry: CustomSentryClient,
    pub logger: Logger,
    pub config_directory: PathBuf,
    pub app_start_time: DateTime<Utc>,
    pub cli: CliRunner,
}

/// Error wrapper of the global object builder.
/// In case of an error handles it
/// and exits the process with code 1.
fn create_global() -> Global {

    create_global_result()
        .crash_on_error()
}

/// Creates the global object.
fn create_global_result() -> Result<Global> {

    let config_directory= ::std::env::var_os("CONFIG_DIRECTORY")
        .map_result(|x| Ok(x.get_as_string()?))?
        .map(|x| Path::new(&x).to_path_buf())
        .unwrap_or_else_result(|| Ok(::std::env::current_exe()?.get_directory()))?;

    let config_file_path = config_directory.join(APP_CONFIG_FILE_NAME);

    if !config_file_path.exists() {

        eprintln!("The `{}` file is missing.", APP_CONFIG_FILE_NAME);

        ::std::process::exit(1);
    }

    let app_config = read_config(&config_file_path.get_as_string()?)
        .crash_on_early_error();

    let sentry = CustomSentryClient::new(&app_config.sentry_dsn)?;

    let log_file_path = config_directory.join(LOG_FILE_NAME);

    let logger = Logger::new(LoggingConfiguration {
        max_length: LOG_FILE_MAX_LENGTH,
        file_path: log_file_path
    })?;

    Ok(Global {
        app_config,
        sentry,
        logger,
        config_directory,
        app_start_time: Utc::now(),
        cli: CliRunner::new()
    })
}

lazy_static! {

    /// The hidden instance reference.
    static ref GLOBAL_INSTANCE: Global = create_global();
}

fn set_panic_hook () {

    ::std::panic::set_hook(Box::new(|info| {

        let backtrace = backtrace::Backtrace::new();

        let message: &str = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &**s,
                None => "Box<Any>",
            },
        };

        let error = CustomError::from_panic_message(message, backtrace);

        handle_error(&error)
            .expect(&format!("An error occurred while handling an error from panic. {:#?}", &error));
    }));
}

/// Runs the initialization code.
/// Should be called first thing in the entry point.
pub fn initialize() {

    ::std::env::set_var("RUST_BACKTRACE", "1");

    set_panic_hook();

    lazy_static::initialize(&GLOBAL_INSTANCE)
}

#[allow(unused)]
pub fn app_config() -> &'static AppConfig {

    &GLOBAL_INSTANCE.app_config
}

#[allow(unused)]
pub fn sentry_client() -> &'static CustomSentryClient {

    &GLOBAL_INSTANCE.sentry
}

#[allow(unused)]
pub fn logger() -> &'static Logger {

    &GLOBAL_INSTANCE.logger
}

#[allow(unused)]
pub fn app_start_time() -> &'static DateTime<Utc> {

    &GLOBAL_INSTANCE.app_start_time
}

#[allow(unused)]
pub fn cli() -> &'static CliRunner {

    &GLOBAL_INSTANCE.cli
}

#[allow(unused_macros)]
macro_rules! log {
    ($x:expr) => {
        crate::global::logger().log(&format!("{}", $x))?
    };
    ($($x:expr),*) => {
        crate::global::logger().log(&format!($($x,)*))?
    };
}

#[allow(unused_macros)]
macro_rules! elog {
    ($x:expr) => {
        crate::global::logger().elog(&format!("{}", $x))?
    };
    ($($x:expr),*) => {
        crate::global::logger().elog(&format!($($x,)*))?
    };
}

#[allow(unused_macros)]
macro_rules! log_error {
    ($x:expr) => {
        global::error_handler::handle_error(&$x.into())?
    };
}
