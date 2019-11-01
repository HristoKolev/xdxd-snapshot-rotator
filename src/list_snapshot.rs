use clap::Arg;

use crate::global::prelude::*;
use crate::snapshot_helper::list_snapshots;

struct ListCommandOptions {
    vm_name: String,
}

fn list_command_options() -> Result<ListCommandOptions> {
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

    Ok(ListCommandOptions {
        vm_name: vm_name.to_string()
    })
}

pub fn list_shapshot_command() -> Result {
    let options = list_command_options()?;

    let config = app_config().snapshot_config.as_ref()
        .and_then(|x| x.get(&options.vm_name).cloned())
        .or_error(&format!("``xdxd-snapshot-rotator` not configured for vm `{}`", options.vm_name))?;

    let snapshots = list_snapshots(&config)?;

    for snapshot in snapshots {
        log!("{} {}", snapshot.vm_name, snapshot.date);
    }

    Ok(())
}

