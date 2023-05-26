use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help(true))]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
#[command(arg_required_else_help(true))]
enum Commands {
    /// Handle libraries
    Library {
        /// Command to run
        #[command(subcommand)]
        command: Option<LibraryCommand>,
    },

    /// Handle the server
    Server {
        /// Command to run
        #[command(subcommand)]
        command: Option<ServerCommand>,
    }
}

#[derive(Subcommand)]
#[command(arg_required_else_help(true))]
enum LibraryCommand {
    /// Create a library
    Create {
        /// Name of library to create
        path: String,
    },

    /// List all libraries
    List {},

    /// Delete a library
    Delete {
        /// Name of library to delete
        name: String,
    },
}

#[derive(Subcommand)]
#[command(arg_required_else_help(true))]
enum ServerCommand {
    /// Start the server
    Run
}

async fn interactive(){

}
#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Library { command }) => match command {
            Some(LibraryCommand::Create { path }) => {
                client_app::create_library(&path, &path, "ws://127.0.0.1:8000");
                println!("Creating library {}", path);
            }
            Some(LibraryCommand::List {}) => {
                let libraries = client_app::get_libraries();
                println!("Name\t\t\tPath");
                for library in libraries {
                    println!("{}\t\t\t{}", library.name, library.path);
                }
            }
            Some(LibraryCommand::Delete { name }) => {
                println!("Deleting library {}", name);
            }
            None => println!("No command specified"),
        },
        Some(Commands::Server { command }) => match command {
            Some(ServerCommand::Run) => server::start_db_server(),
            None => {},
        }
        None => println!("No command specified"),
    }
}
