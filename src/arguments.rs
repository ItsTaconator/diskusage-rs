use clap::Parser;

use crate::style::Style;

/// Disk usage utility for Linux
#[derive(Clone, Parser, Debug)]
#[command(version, about, after_help = "Be careful with enabling too many display options; table *will* wraparound")]
pub(crate) struct Arguments {
    /// Shows filesystems
    #[arg(long, short)]
    pub fs: bool,
    /// Shows block devices
    #[arg(long, short)]
    #[cfg(unix)]
    pub devices: bool,
    /// Shows partition UUIDs
    #[arg(long, short)]
    #[cfg(unix)]
    pub uuid: bool,
    /// Shows mount options
    #[arg(long, short = 'o')]
    #[cfg(unix)]
    pub mount_options: bool,
    #[arg(long, short)]
    pub color: bool,
    /// Output table style, see https://docs.rs/tabled/latest/tabled/settings/style/struct.Style.html
    #[arg(long, value_enum, default_value_t = Style::Modern)]
    pub style: Style,
    /// Show /boot partition
    #[arg(long)]
    pub boot: bool,
    /// Shows disk usage as a bar along with values
    #[arg(long, short)]
    pub bar: bool,
    /// How many segments the disk usage bar should have
    #[arg(long, short, default_value_t = 0)]
    pub segments: u8,
}
