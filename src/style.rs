use clap::ValueEnum;

#[derive(Clone, ValueEnum, Debug)]
pub(crate) enum Style {
    Ascii,
    AsciiRounded,
    Blank,
    Dots,
    Empty,
    Extended,
    Markdown,
    Modern,
    ModernRounded,
    Psql,
    ReStructuredText,
    Sharp,
}
