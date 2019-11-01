use crate::global::prelude::*;
use chrono::{DateTime, Utc, TimeZone};

#[derive(Debug)]
pub struct VmSnapshot {
    pub vm_name: String,
    pub date: DateTime<Utc>,
    pub snapsnot_name: String,
}

pub fn list_snapshots(config: &VmConfig) -> Result<Vec<VmSnapshot>> {
    let ps = bash_exec_no_log!("virsh snapshot-list --domain {} --internal", config.vm_name);

    let lines: Vec<&str> = ps.stdout
        .split('\n')
        .skip(2)
        .filter(|x| x != &"")
        .collect_vec();

    let snapshots = lines.into_iter()
        .map(|line| {
            let line_parts: Vec<&str> = line.split(' ')
                .filter(|x| x != &"")
                .collect_vec();

            line_parts.get(0).and_then(|snapsnot_name| {
                let parts: Vec<&str> = snapsnot_name.split('.').collect_vec();

                parts.get(parts.len() - 1)
                    .and_then(|x| x.parse::<i64>().ok())
                    .map(|x| Utc.timestamp(x, 0))
                    .map(|x| VmSnapshot {
                        vm_name: parts[0].to_string(),
                        date: x,
                        snapsnot_name: snapsnot_name.to_string()
                    })
            })
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect_vec();

    Ok(snapshots)
}

pub fn clear_cache(config: &VmConfig) -> Result {
    let snapshots = list_snapshots(config)?;

    let take_count = if ((snapshots.len() as i32) - config.min_snapshot_count) < 0 {
        0
    } else {
        (snapshots.len() as i32) - config.min_snapshot_count
    };

    let for_delete = snapshots
        .into_iter()
        .order_by(|x| x.date)
        .take(take_count as usize)
        .collect_vec();

    for snapshot in for_delete {
        log!("Deleting snapshot `{}` ...", snapshot.snapsnot_name);

        bash_exec!("virsh snapshot-delete --domain {} --snapshotname {}", snapshot.vm_name, snapshot.snapsnot_name);
    }

    bash_exec_no_log!("virsh snapshot-list --domain {} --internal", config.vm_name);

    Ok(())
}