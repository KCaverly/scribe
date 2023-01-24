mod note;
mod path;

use std::env;
use std::process::exit;

use clap::{Parser, Subcommand};
use lazy_static::lazy_static;
use note::{Note, NoteManager};

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "scribe")
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

        /// Launch in Interactive Mode
        #[clap(short, long, action)]
        interactive: bool,

        /// Launch editor after creation
        #[clap(short, long, action)]
        edit: bool,
    },

    /// Transfer a note to a new category
    Transfer {
        #[clap(short, long)]
        /// The path of the note to transfer
        path: Option<String>,

        #[clap(short, long)]
        /// The category of the note to transfer
        category: Option<String>,

        /// Launch in Interactive Mode
        #[clap(short, long, action)]
        interactive: bool,
    },

    /// Archive a note
    Archive {
        #[clap(short, long)]
        /// The path of the note to transfer
        path: String,
    },

    /// Search
    Search {
        /// The Words to Search on
        search_string: String,
    },

    /// Sync Notes to Git
    Sync {
        /// The Commit Message to Use
        commit_message: Option<String>,
    },
}

lazy_static! {
    #[derive(Debug)]
    static ref NOTES_DIR: String = env::var("NOTES_DIR").unwrap();
}

fn main() {
    if env::var("NOTES_DIR").is_err() {
        println!("Please set NOTES_DIR before continuing.");
        exit(0)
    }

    let args = Cli::parse();

    match args.command {
        Commands::New {
            name,
            category,
            tags,
            interactive,
            edit,
        } => {
            if interactive {
                let note = NoteManager::interactive_create();
                if edit {
                    note.edit();
                }

                exit(0);
            }

            if !name.is_some() {
                println!("Please provide a name for your note: `pj new --name <name>`");
                exit(0);
            }

            let tags_vec = Note::parse_tags(tags);

            let note = Note::new(category, name.unwrap(), tags_vec, None);
            note.init();

            if edit {
                note.edit();
            }
        }

        Commands::Transfer {
            path,
            category,
            interactive,
        } => {
            if interactive {
                NoteManager::interactive_transfer();
                exit(0);
            }

            if path.is_none() {
                println!(
                    "Please provide a path for the file to transfer: `pj transfer --path <path>`"
                );
                exit(0);
            }

            if category.is_none() {
                println!(
                    "Please provide a category to transfer to: `pj transfer --category <category>`"
                );
                exit(0);
            }

            _ = NoteManager::transfer(&path.unwrap(), &category.unwrap());
        }

        Commands::Archive { path } => {
            _ = NoteManager::transfer(&path, "archive");
        }

        Commands::Search { search_string } => {
            let search_results = NoteManager::search_notes(search_string);
            for res in search_results.unwrap() {
                println!("{}  {}", res.0, res.1);
            }
        }

        Commands::Sync { commit_message } => {
            NoteManager::pull();
            if commit_message.is_some() {
                _ = NoteManager::save(&commit_message.unwrap());
            } else {
                _ = NoteManager::save("(pj) Saving Notes...");
            }
        }
    }
}
