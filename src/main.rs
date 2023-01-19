mod note;

use clap::{Parser, Subcommand};
use note::{Note, NoteManager};

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
        name: String,

        #[clap(short, long)]
        /// The Category of the new note.
        category: Option<String>,

        // TODO: Change this to be a proper vector
        #[clap(short, long)]
        /// Tags associated with the new note, pass each seperated by a ,
        tags: Option<String>,
    },

    /// Transfer a note to a new category
    Transfer {
        #[clap(short, long)]
        /// The path of the note to transfer
        path: String,

        #[clap(short, long)]
        /// The category of the note to transfer
        category: String,
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

    /// Save Notes to Git
    Save {
        /// The Commit Message to Use
        commit_message: Option<String>,
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
            let tags_vec: Option<Vec<String>>;
            if tags.is_some() {
                let t = tags.unwrap();
                let vec: Vec<&str> = t.split(",").collect();
                let mut tv = Vec::<String>::new();
                for v in vec {
                    tv.push(v.to_string());
                }
                tags_vec = Some(tv);
            } else {
                tags_vec = None;
            };

            let note = Note::new(category, name, tags_vec, None);
            note.init();
        }

        Commands::Transfer { path, category } => {
            _ = Note::transfer(path, category);
        }

        Commands::Archive { path } => {
            _ = Note::transfer(path, "archive".to_string());
        }

        Commands::Search { search_string } => {
            let search_results = NoteManager::search_notes(search_string);
            for res in search_results.unwrap() {
                println!("{}  {}", res.0, res.1);
            }
        }

        Commands::Save { commit_message } => {
            if commit_message.is_some() {
                _ = NoteManager::save_notes(commit_message.unwrap());
            } else {
                _ = NoteManager::save_notes("(pj) Saving Notes...".to_string());
            }
        }
    }
}
