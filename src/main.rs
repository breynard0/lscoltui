pub mod colours;
pub mod file;
pub mod key_events;
pub mod ui;

use std::{
    io::stdout,
    panic::{set_hook, take_hook},
};

use clap::{Parser, Subcommand};
use crossterm::{execute, terminal::disable_raw_mode};
use file::SaveFile;
use ui::App;

#[derive(Parser)]
#[command(name = "lscoltui")]
#[command(about = "A TUI tool for configuring the colours of ls", long_about = None)]
struct Cli {
    #[command[subcommand]]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Prints the command to set the colours. Recommended to chain with eval $()")]
    Export { name: Option<String> },
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    let savefile = SaveFile::load();

    if cli.command.is_some() {
        match &cli.command.unwrap() {
            Commands::Export { name } => match name {
                Some(s) => {
                    match savefile.schemes.iter().find(|x| &x.0 == s) {
                        Some(scheme) => println!("{}", file::env_command(&scheme.1)),
                        None => {
                            eprintln!("Unable to find scheme \'{}\'", s)
                        }
                    }
                    return Ok(());
                }
                None => {
                    match savefile
                        .schemes
                        .iter()
                        .find(|x| x.0 == savefile.most_recent)
                    {
                        Some(scheme) => println!("{}", file::env_command(&scheme.1)),
                        None => {}
                    }
                    return Ok(());
                }
            },
        }
    }

    init_panic_hook();
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let mut app = App::default();
    if !app.savefile.most_recent.is_empty()
        && app
            .savefile
            .schemes
            .iter()
            .find(|x| x.0 == app.savefile.most_recent)
            .is_some()
    {
        app.open_scheme = Some(app.savefile.most_recent.clone());
    }
    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}

pub fn init_panic_hook() {
    println!("Quit signal recieved, app shutting down");
    let original_hook = take_hook();
    set_hook(Box::new(move |panic_info| {
        let _ = restore_tui();
        original_hook(panic_info);
    }));
}

pub fn restore_tui() -> std::io::Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), crossterm::terminal::LeaveAlternateScreen)?;
    Ok(())
}
