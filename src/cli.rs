use crate::scribe::Scribe;
use crate::note::Note;
use clap::{Parser, Subcommand};
use std::process::exit;

#[derive(Debug, Parser)]
#[command(name = "scribe")]
#[command(about = "A Smart Notes Manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {

    /// Create a new note
    New {
        /// The title of the new note
        #[clap(short, long)]
        name: Option<String>,

        /// The Category of the new note
        #[clap(short, long)]
        category: Option<String>,

        /// Tags associated with the new note, pass each seperated by a ','
        #[clap(short, long)]
        tags: Option<String>,

        /// Launch in Interactive Mode
        #[clap(short, long, action)]
        interactive: bool,

        /// Launch editor after creation
        #[clap(short, long, action)]
        edit: bool

    },

    /// Transfer a note to a new category
    Transfer {
        /// The path of the note to transfer
        #[clap(short, long)]
        path: Option<String>,

        /// The category of the note to transfer
        #[clap(short, long)]
        category: Option<String>,

        /// Launch in Interactive Mode
        #[clap(short, long, action)]
        interactive: bool,
    },

    /// Archive a note
    Archive {
        /// The path of the note to transfer
        path: String,
    },

    /// Search Notes
    Search {
        /// The Words to Search On
        #[clap(short, long)]
        search_string: String
    },

    /// Sync Notes to Git
    Sync {
        /// The Commit Message to Use
        #[clap(short, long)]
        commit_message: Option<String>,
    }

}

fn parse_cli_tags(tags: Option<String>) -> Option<Vec<String>> {

    let tags_vec: Option<Vec<String>>;
    if tags.is_some() {
        let t = tags.unwrap();
        let vec: Vec<&str> = t.split(",").collect();
        let mut tv = Vec::<String>::new();
        for v in vec {
            tv.push(v.trim().to_string());
        }
        tags_vec = Some(tv);
    } else {
        tags_vec = None; 
    };

    return tags_vec
    
}

pub fn run_cli() {

    let args = Cli::parse();

    match args.command {
        
        Commands::New {
            name,
            category,
            tags,
            interactive,
            edit,
        } => {
            let note: Note;
            if interactive {
                note = Scribe::interactive_create();
            } else {
                if !name.is_some() {
                    println!("Please provide a title for your note: `scribe new --name <name>`");
                    exit(0);
                }

                // TODO: I Dont think we should be access Note here
                let tags_vec = parse_cli_tags(tags);
                note = Scribe::create(name.unwrap(), category, tags_vec);
            }

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
                Scribe::interactive_transfer();
            } else {
                if path.is_none() {
                    println!("Please provide a path for the file to transfer: `scribe transfer --path <path>`");
                    exit(0);
                }

                if category.is_none() {
                    println!("Please provide a category to transfer to: `scribe transfer --category <category>`");
                    exit(0);
                }

                Scribe::transfer(&path.unwrap(), &category.unwrap());

            }

        }

        Commands::Archive { path } => {
            Scribe::transfer(&path, "archive");
        }

        Commands::Search {search_string } => {
            let search_results = Scribe::search(&search_string);
            for res in search_results.unwrap() {
                println!("{}  {}", res.0, res.1);
            }
        }

        Commands::Sync { commit_message } => {
            Scribe::sync(commit_message);
        }


    }
}
