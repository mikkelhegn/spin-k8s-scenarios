use anyhow::Error;
use serde::{Deserialize, Serialize};
use std::io::{self};

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    widgets::{Block, ListItem, ListState, Padding, Paragraph},
    DefaultTerminal, Frame,
};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = run(&mut terminal);
    ratatui::restore();
    app_result
}

fn run(terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| draw(frame))?;
        if handle_events()? {
            break Ok(());
        }
    }
}

fn draw(frame: &mut Frame) {
    let text = Paragraph::new("Hello World!");
    frame.render_widget(text, frame.area());
}

fn handle_events() -> std::io::Result<bool> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('q') => return Ok(true),
            // handle other key events
            _ => {}
        },
        // handle other events
        _ => {}
    }
    Ok(false)
}

fn load_scenario() -> Result<Snippet, Error> {
    let read_file = std::fs::read_to_string("snip-k3d-create.toml").expect("Failed to read file");

    let snippet_file: SnippetFile =
        toml::from_str(&read_file.to_string()).expect("Failed to parse to TOML");

    Ok(snippet_file.snippet)
}

fn run_scenario(selected: Option<usize>) {
    println!("You selected {:?}", selected.expect("Nothing selected"));
}

// Structs
#[derive(Copy, Clone)]
struct Scenario<'a> {
    name: &'a String,
    order_of_snippets: &'a (u8, Vec<Snippet>),
}

impl<'a> From<Scenario<'a>> for ListItem<'a> {
    fn from(val: Scenario<'a>) -> Self {
        ListItem::new(val.name.as_str())
    }
}

// Snippets has commands, resources, variables, and prerequisites
#[derive(Serialize, Deserialize, Debug)]
struct Snippet {
    name: String,
    description: String,
    commands: Vec<Command>,
    variables: Vec<Variable>,
    //resource_files_names: Option<Vec<String>>,
    //prerequisites: Option<Vec<Snippet>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SnippetFile {
    snippet_version: String,
    snippet: Snippet,
}

#[derive(Serialize, Deserialize, Debug)]
struct Command {
    name: String,
    command: String,
}

// TODO: How can this be a tuple of (String, String)?
// TODO: Can the value be a number in Toml, but parsed to string with serde?
#[derive(Serialize, Deserialize, Debug)]
struct Variable {
    name: String,
    value: String,
}
