use clap::Arg;

use crate::global::prelude::*;
use crate::snapshot_helper::clear_cache;

struct CreateCommandOptions {
    vm_name: String,
}

fn create_command_options() -> Result<CreateCommandOptions> {

    const VM_NAME_VALUE: &str = "vm-name";

    let matches = cli().command_config(|x| {

        x.arg(Arg::with_name(VM_NAME_VALUE)
            .short("n")
            .long(VM_NAME_VALUE)
            .value_name(VM_NAME_VALUE)
            .help("The name of virtual machine.")
            .required(true)
            .takes_value(true)
        )
    });

    let vm_name = matches.value_of(VM_NAME_VALUE)
        .or_error(&format!("No value for: {}", VM_NAME_VALUE))?;

    Ok(CreateCommandOptions {
        vm_name: vm_name.to_string()
    })
}

pub fn create_shapshot_command() -> Result {

    let options = create_command_options()?;

    let config = app_config().snapshot_config.as_ref()
        .and_then(|x| x.get(&options.vm_name).cloned())
        .or_error(&format!("``xdxd-snapshot-rotator` not configured for vm `{}`", options.vm_name))?;

    let now = app_start_time();

    let snapshot_name = format!(
        "{}.{}.{}",
        config.vm_name,
        now.format("%Y-%m-%d_%H-%M-%S").to_string(),
        now.timestamp().to_string()
    );

    bash_exec!("virsh snapshot-create-as {} --name {}", config.vm_name, snapshot_name);

    clear_cache(&config)?;

    email_report::send_success_report(&config)?;

    Ok(())
}

