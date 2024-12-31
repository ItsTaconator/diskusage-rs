use clap::Parser;

use crate::style::Style;

#[derive(Clone, Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Arguments {
    /// Shows filesystems
    #[arg(long)]
    pub fs: bool,
    /// Shows block devices
    #[arg(long, short)]
    pub devices: bool,
    /// Shows partition UUIDs
    #[arg(long)]
    pub uuid: bool,
    /// Shows mount options
    #[arg(long, short = 'o')]
    pub mountoptions: bool,
    /// Enables color
    #[arg(long, short)]
    pub color: bool,
    /// Output table style, see https://docs.rs/tabled/latest/tabled/settings/style/struct.Style.html
    #[arg(long, short, value_enum, default_value_t = Style::Modern)]
    pub style: Style,
    /// Show /boot partition
    #[arg(long)]
    pub boot: bool,
}
