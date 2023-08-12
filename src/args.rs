use clap::Parser;

/// Advanced Navigational Text Explorer
#[derive(Parser, Debug)]
#[command(author = "DigitalCyan", version = "1.0.0", about = "Advanced Navigational Text Explorer")]
pub struct Args {
    
    /// The path to the folder from which we began the recursive search
    #[arg()]
    pub path: String,

    /// The piece of text to look for
    #[arg(short, long)]
    pub text: Option<String>,

    /// A RegEx to look for
    #[arg(short, long)]
    pub regex: Option<String>,
}