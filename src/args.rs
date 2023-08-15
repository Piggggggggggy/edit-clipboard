use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Use [csuqx]
    /// This is a list of filter flags inorder
    #[arg(short, long)]
    pub filter: Option<String>,
}
