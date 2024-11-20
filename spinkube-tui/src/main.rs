use anyhow::Error;
use serde::{Deserialize, Serialize};
use std::io;

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    widgets::{Block, Borders, List, ListItem, ListState, Padding, Paragraph},
    Frame,
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
    // SnippetsList state
    let mut snippets_list = SnippetsList::new(load_snippets().expect("Failed to get scnearios"));

    // Layout
    let [top, main, bottom] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Fill(2),
        Constraint::Length(5),
    ])
    .margin(2)
    .flex(Flex::Center)
    .areas(frame.area());

    let top = Layout::horizontal([Constraint::Percentage(100)])
        .flex(Flex::Center)
        .split(top);

    let main =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(main);

    let bottom = Layout::horizontal([Constraint::Percentage(100)])
        .flex(Flex::Center)
        .split(bottom);

    let block = Block::default().borders(Borders::ALL);

    let text_top = Paragraph::new("Hello World!").centered();
    let text_temp = Paragraph::new("Hello Bottom!").block(block.clone().title("Scenarios"));
    let list: List<'_> = List::new(snippets_list.snippets).block(block.clone().title("Snippets"));
    // TODO split bottom in two and create two bottons
    let text_bottom = Paragraph::new("Hello Bottom!").centered();

    frame.render_widget(text_top, top[0]);
    frame.render_widget(text_temp, main[0]);
    frame.render_stateful_widget(list, main[1], &mut snippets_list.state);
    frame.render_widget(text_bottom, bottom[0]);
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
