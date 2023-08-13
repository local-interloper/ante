use clap::Parser;

/// Advanced Navigational Text Explorer
#[derive(Parser, Debug)]
#[command(author = "DigitalCyan", version = "1.0.0", about = "Advanced Navigational Text Explorer")]
pub struct Args {
    /// The path to the folder from which we begin the recursive search
    #[arg()]
    pub path: String,

    /// The piece of text to look for
    #[arg(short, long)]
    pub text: Option<String>,

    /// A RegEx to look for
    #[arg(short, long)]
    pub regex: Option<String>,
    
    /// Only print paths to files that match the criteria
    #[arg(short, long)]
    pub paths_only: bool,
    
    /// Max file size
    #[arg(short, long)]
    pub max_file_size: Option<u64>
}