use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Build {
        #[arg(long)]
        prev: Option<String>,
        #[arg(long)]
        scope: String,
        #[arg(long)]
        author: String,
        #[arg(long)]
        usher: String,
        #[arg(long)]
        schema: Option<String>,
        #[arg(long)]
        rt: String,
        #[arg(long)]
        data: Option<String>,
        output: String,
    },
    Finalize {
        #[arg(short, long)]
        input: String,
        #[arg(short, long)]
        output: String,
        #[arg(short, long)]
        use_curr: bool,
    },
    Genesis {
        #[arg(short, long)]
        key: String,
        #[arg(short, long)]
        output: String,
        #[arg(short, long)]
        enclave_path: Option<String>,
    },
    View {
        #[arg(short, long)]
        input: String,
    },
    Submit {
        #[arg(long)]
        rhex: String,
        #[arg(short, long)]
        address: Option<String>,
        #[arg(short, long)]
        request: Option<String>,
    },
}
