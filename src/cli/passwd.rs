use clap::Args;

#[derive(Debug, Args)]
pub struct PassGenOpts {
    #[arg(long, default_value_t = 12, help = "Length of the password")]
    pub length: u8,

    #[arg(long, action = clap::ArgAction::SetTrue, help = "Disable lowercase letters")]
    pub no_lowercase: bool,

    #[arg(long, action = clap::ArgAction::SetTrue, help = "Disable uppercase letters")]
    pub no_uppercase: bool,

    #[arg(long, action = clap::ArgAction::SetTrue, help = "Disable numbers")]
    pub no_numbers: bool,

    #[arg(long, action = clap::ArgAction::SetTrue, help = "Disable symbols")]
    pub no_symbols: bool,
}