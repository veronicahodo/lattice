use clap::{Parser, Subcommand};

pub use key::Key;
pub use rhex::Rhex;

mod generate;
mod import;
mod sign;
mod vanity;
mod view;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// This is the key in base64 format that we will be using
    /// to pull from the enclave
    #[arg(short, long, global = true)]
    key: Option<String>,

    #[arg(long, global = true)]
    enclave_path: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Sign {
        /// Signature Type
        #[arg(short, long)]
        sig_type: String,

        /// Path to Rhex for signing/verifying
        #[arg(short, long)]
        input: String,

        /// Path to Rhex for signing
        #[arg(short, long, global = true)]
        output: Option<String>,

        /// Delta of how long since usher's added `at`
        #[arg(short, long)]
        delta: Option<u64>,
    },
    Generate {
        #[arg(short, long)]
        name: Option<String>,
        #[arg(short, long)]
        time: Option<u64>,
        #[arg(short, long)]
        expires: Option<u64>,
    },
    View {
        #[arg(long)]
        secret: bool,

        #[arg(long)]
        rust: bool,
    },
    Vanity {
        #[arg(short, long)]
        sigil_prefix: String,
        #[arg(short, long)]
        name: Option<String>,
        #[arg(short, long)]
        time: Option<u64>,
        #[arg(long)]
        expires: Option<u64>,
    },
    Import {
        #[arg(short, long)]
        name: Option<String>,
        #[arg(short, long)]
        time: Option<u64>,
        #[arg(short, long)]
        expires: Option<u64>,
    },
}

fn main() {
    println!("Lattice Key Tool v{}", env!("CARGO_PKG_VERSION"));
    let cli = Cli::parse();
    let enclave_path = match cli.enclave_path {
        Some(ep) => ep,
        None => "./keys".to_string(),
    };
    match cli.command {
        Commands::Sign {
            sig_type,
            input,
            output,
            delta,
        } => {
            if cli.key.is_none() {
                println!("Must specify key in base64 format (-k)");
                return;
            }

            let _ = sign::sign(
                &cli.key.unwrap(),
                &enclave_path,
                &sig_type,
                &input,
                &output,
                &delta,
            );
        }
        Commands::Generate {
            name,
            time,
            expires,
        } => {
            let _ = generate::generate(name, cli.key.unwrap(), time, expires);
        }
        Commands::View { secret, rust } => {
            view::view(&cli.key.unwrap(), enclave_path, secret, rust);
        }
        Commands::Vanity {
            sigil_prefix,
            name,
            time,
            expires,
        } => {
            let _ = vanity::vanity(sigil_prefix, name, time, expires, cli.key.unwrap());
        }
        Commands::Import {
            name,
            time,
            expires,
        } => {
            if cli.key.is_none() {
                println!("Must specify key in base64 format (-k)");
                return;
            }

            let _ = import::import(&enclave_path, &name, &cli.key.unwrap(), &time, &expires);
        }
    }
}
