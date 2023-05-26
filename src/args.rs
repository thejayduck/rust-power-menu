use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Arguments {
    /// Disables icons.
    #[clap(long, short, action)]
    pub no_icon: bool,
    /// Preview power menu actions without execution.
    #[clap(long, short, action)]
    pub dry_run: bool,
}
