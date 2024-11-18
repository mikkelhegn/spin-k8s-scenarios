use anyhow::Error;
use serde::{Deserialize, Serialize};
use std::io::{self};

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    widgets::{Block, List, ListItem, ListState, Padding, Paragraph},
    DefaultTerminal, Frame,
};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = run(&mut terminal);
    ratatui::restore();
    app_result
}

struct SnippetsList {
    snippets: Vec<Snippet>,
    state: ListState,
}

impl SnippetsList {
    fn new(snippets: Vec<Snippet>) -> SnippetsList {
        SnippetsList {
            snippets,
            state: ListState::default(),
        }
    }
}

fn run(terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(draw)?;
        if handle_events()? {
            break Ok(());
        }
    }
}

fn draw(frame: &mut Frame) {
    let mut snippets_list = SnippetsList::new(load_snippets().expect("Failed to get scnearios"));

    let text = Paragraph::new("Hello World!");
    let list: List<'_> = List::new(snippets_list.snippets);

    frame.render_widget(text, frame.area());
    frame.render_stateful_widget(list, frame.area(), &mut snippets_list.state);
}

fn handle_events() -> std::io::Result<bool> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('q') => return Ok(true),
            _ => {}
        },
        _ => {}
    }
    Ok(false)
}

fn load_snippets() -> Result<Vec<Snippet>, Error> {
    let read_file = std::fs::read_to_string("snip-k3d-create.toml").expect("Failed to read file");

    let snippet_file: SnippetFile =
        toml::from_str(&read_file.to_string()).expect("Failed to parse to TOML");

    let snippets: Vec<Snippet> = vec![snippet_file.snippet];

    Ok(snippets)
}

fn run_scenario(selected: Option<usize>) {
    println!("You selected {:?}", selected.expect("Nothing selected"));
}

// Snippets has commands, resources, variables, and prerequisites
#[derive(Clone, Serialize, Deserialize, Debug)]
struct Snippet {
    name: String,
    description: String,
    commands: Vec<Command>,
    variables: Vec<Variable>,
    //resource_files_names: Option<Vec<String>>,
    //prerequisites: Option<Vec<Snippet>>,
}

impl From<Snippet> for ListItem<'_> {
    fn from(val: Snippet) -> Self {
        ListItem::new(val.name)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct SnippetFile {
    snippet_version: String,
    snippet: Snippet,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Command {
    name: String,
    command: String,
}

// TODO: How can this be a tuple of (String, String)?
// TODO: Can the value be a number in Toml, but parsed to string with serde?
#[derive(Clone, Serialize, Deserialize, Debug)]
struct Variable {
    name: String,
    value: String,
}
