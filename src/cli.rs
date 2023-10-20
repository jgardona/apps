mod error;
mod fsystem;
mod view;

use askama::Template;
use clap::{Parser, Subcommand};

use crate::cli::{
    fsystem::{get_dir_iter, DesktopTemplate, remove_file},
    view::{display, FileItem},
};

use self::fsystem::write_file;

#[derive(Subcommand, Debug)]
enum Commands {
    /// Reads data from application's folder
    Read {
        /// Lists all installed applications
        #[arg(short, long)]
        list: bool,
        /// Counts the number of application
        #[arg(short, long)]
        count: bool,
    },
    /// Creates a new application launcher
    Create {
        /// The application's name
        name: String,
        /// The application's icon
        icon: String,
        /// The application's executable
        exec: String,
        /// The comments about the application
        comment: String,
    },

    /// Removes an application launcher from folder
    Remove {
        /// The application's name
        name: String,
    },
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    // Commands to manage application's folder
    #[command(subcommand)]
    command: Commands,
}

fn match_commands(command: Commands) -> error::Result<()> {
    match command {
        Commands::Read { list, count } => {
            if list {
                let fdata = get_dir_iter()?;
                let buffer = cast(fdata)?;
                let out = display(buffer)?;
                println!("{out}");
            }

            if count {
                let fdata = get_dir_iter()?;
                let size = cast(fdata)?.len();
                println!("Number of local applications: {size}");
            }
        }
        Commands::Create {
            name,
            icon,
            exec,
            comment,
        } => {
            let template = DesktopTemplate::new(
                name.as_str(),
                icon.as_str(),
                exec.as_str(),
                comment.as_str(),
            );

            let rendered = template.render()?;
            write_file(template.appname, &rendered)?;
        }
        Commands::Remove { name } => {
            remove_file(&name)?;
            println!("Application removed");
        }
    }
    Ok(())
}

fn cast(fdata: std::fs::ReadDir) -> error::Result<Vec<FileItem>> {
    let mut buffer = vec![];
    for e in fdata {
        let e = e?;
        let filename = e.file_name().clone();
        let filename: String = filename.to_str().unwrap().into();
        let kind = if !filename.ends_with("desktop") {
            "✕".to_string()
        } else {
            "✔".to_string()
        };
        let item = FileItem::new(kind, filename);
        buffer.push(item);
    }
    Ok(buffer)
}

pub fn execute() -> error::Result<()> {
    let cli = Cli::parse();
    match_commands(cli.command)?;
    Ok(())
}
