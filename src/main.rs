#![deny(unsafe_code)]

#[link(name="openssl", kind="static")]
extern crate openssl;

#[macro_use]
mod global;

mod config;
mod create_snapshot;
mod list_snapshot;
mod snapshot_helper;
mod clear_cache;

use crate::global::prelude::*;
use crate::global::errors::CustomErrorKind;

use crate::config::config_command;
use crate::create_snapshot::{create_shapshot_command};
use crate::list_snapshot::list_shapshot_command;
use crate::clear_cache::clear_cache_command;

fn main() {

    global::initialize();

    main_result().crash_on_error();
}

fn main_result() -> Result {

    cli().register_command("clear-cache", Box::new(clear_cache_command))?;
    cli().register_command("list", Box::new(list_shapshot_command))?;
    cli().register_command("create", Box::new(create_shapshot_command))?;
    cli().register_command("config", Box::new(config_command))?;

    match cli().run() {
        Err(err) => {
            if let CustomErrorKind::UserError(message) = err.kind {
                log!("Error: {}", message);
                ::std::process::exit(1);
            } else {
                return Err(err);
            }
        },
        Ok(_) => ()
    };

    Ok(())
}
