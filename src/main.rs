mod note;

use clap::{Args, Parser, Subcommand};
use note::Note;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "pj")]
#[command(about = "A Smart Notes Manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Create a new note
    New {
        #[clap(short, long)]
        /// The title of the new note
        name: Option<String>,

        #[clap(short, long)]
        /// The Category of the new note.
        category: Option<String>,

        // TODO: Change this to be a proper vector
        #[clap(short, long)]
        /// Tags associated with the new note, pass each seperated by a ,
        tags: Option<String>,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::New {
            name,
            category,
            tags,
        } => {
            // println!("NAME: {}", name.unwrap());
            // println!("CATEGORY: {}", category.unwrap());
            // println!("TAGS: {}", tags.unwrap());
            Note::new(&category.unwrap(), &name.unwrap())
        }
    }
}
