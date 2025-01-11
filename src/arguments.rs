use clap::Parser;

use crate::style::Style;

#[derive(Clone, Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Arguments {
    /// Shows filesystems
    #[arg(long, short)]
    pub fs: bool,
    /// Shows block devices
    #[arg(long, short)]
    pub devices: bool,
    /// Shows partition UUIDs
    #[arg(long, short)]
    pub uuid: bool,
    /// Shows mount options
    #[arg(long, short = 'o')]
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
    #[arg(long, short, default_value_t = 10)]
    pub segments: u8,
}
