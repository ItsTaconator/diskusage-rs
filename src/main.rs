mod arguments;
mod style;
mod bar;

use std::{collections::HashMap, fs};

use arguments::Arguments;
use bytesize::ByteSize;
use clap::Parser;
use tabled::{builder::Builder, settings::Style};

const RESET: &str = "\x1b[0m";
const BLUE: &str = "\x1b[34m";
const RED: &str = "\x1b[31m";
const BOLD: &str = "\x1b[1m";

fn main() {
    let args = Arguments::parse();

    let mut uuid_map = HashMap::new();

    for uuid in fs::read_dir("/dev/disk/by-uuid/").unwrap() {
        let uuid = uuid.unwrap();
        let actual_path = fs::read_link(uuid.path()).unwrap();
        uuid_map.insert(
            actual_path
                .to_str()
                .unwrap()
                .replace("../../", "/dev/")
                .to_owned(),
            uuid.file_name().into_string().unwrap(),
        );
    }

    let mounts = procfs::mounts().unwrap();

    let disks = sysinfo::Disks::new_with_refreshed_list();
    let mut builder = Builder::default();
    for disk in disks.list() {
        let mountpoint = disk.mount_point().to_str().unwrap().to_string();
        if !args.boot && mountpoint == "/boot" {
            continue;
        }

        let total_space = ByteSize(disk.total_space()).to_string();
        let free_space = ByteSize(disk.available_space()).to_string();
        let used_space = ByteSize(disk.total_space() - disk.available_space()).to_string();
        let device = disk.name().to_str().unwrap().to_owned();
        let filesystem = disk.file_system().to_str().unwrap().to_owned();

        let low = (disk.total_space() as f32 * 0.10) > disk.available_space() as f32;

        let mut output = Vec::<String>::with_capacity(4);

        output.push(mountpoint.clone());
        if args.devices {
            output.push(device.clone());
        }

        if args.fs {
            output.push(filesystem);
        }

        if args.uuid {
            let uuid = uuid_map.get(&device);
            if let Some(uuid) = uuid {
                output.push(uuid.to_string());
            }
        }

        if args.mountoptions {
            for mount in &mounts {
                if mount.fs_file == mountpoint {
                    let mut result = mount
                        .fs_mntops
                        .iter()
                        .map(|m| {
                            if m.1.is_none() {
                                m.0.clone()
                            } else {
                                format!("{}={}", *m.0, m.1.clone().unwrap())
                            }
                        })
                        .collect::<Vec<String>>();

                    result.sort();
                    output.push(result.join(","));
                }
            }
        }

        if args.bar {
            let percent_full = ((disk.total_space() as f64 - disk.available_space() as f64) / disk.total_space() as f64) * 100f64;
            output.push(
                format!(
                    "{}{} {:.1}%, {}/{}{RESET}",
                    if low { RED } else { "" },
                    bar::generate(percent_full as u64, args.segments),
                    percent_full,
                    used_space,
                    total_space
                )
            );
        } else {
            output.push(total_space);
            output.push(used_space);
            if args.color && low {
                output.push(format!("{RED}{free_space}{RESET}"));
            } else {
                output.push(free_space);
            }
        }

        builder.push_record(output);
    }

    let mut columns = if args.bar {
        vec!["Mountpoint", "Space"]
    } else {
        vec!["Mountpoint", "Total", "Used", "Free"]
    };

    let mut i = 1;
    if args.devices {
        columns.insert(i, "Device");
        i += 1;
    }

    if args.fs {
        columns.insert(i, "Filesystem");
        i += 1;
    }

    if args.uuid {
        columns.insert(i, "UUID");
        i += 1;
    }

    if args.mountoptions {
        columns.insert(i, "Mount Options");
        // i += 1;
    }

    let columns = if args.color {
        columns
            .iter()
            .map(|c| format!("{BLUE}{BOLD}{c}{RESET}"))
            .collect::<Vec<String>>()
    } else {
        columns.iter().map(|s| s.to_string()).collect()
    };

    builder.insert_record(0, columns);

    let mut table = builder.build();

    println!(
        "{}",
        match args.style {
            style::Style::Ascii => table.with(Style::ascii()),
            style::Style::AsciiRounded => table.with(Style::ascii_rounded()),
            style::Style::Blank => table.with(Style::blank()),
            style::Style::Dots => table.with(Style::dots()),
            style::Style::Empty => table.with(Style::empty()),
            style::Style::Extended => table.with(Style::extended()),
            style::Style::Markdown => table.with(Style::markdown()),
            style::Style::Modern => table.with(Style::modern()),
            style::Style::ModernRounded => table.with(Style::modern_rounded()),
            style::Style::Psql => table.with(Style::psql()),
            style::Style::ReStructuredText => table.with(Style::re_structured_text()),
            style::Style::Sharp => table.with(Style::sharp()),
        }
    );
}
