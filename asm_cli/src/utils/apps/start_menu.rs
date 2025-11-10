use ratatui::{
    widgets::{List, ListItem, Block, Borders, Paragraph},
    style::{Style, Color, Modifier, Styled},
    text::{Text, Line, Span},
    layout::{Layout, Constraint, Direction, Rect},
    Frame,
};
use crossterm::event::{KeyEvent, KeyCode, KeyEventKind};
use crate::utils::ui::common::{AppState, centered_rect};
use crate::utils::workspaces::Workspace;
use crate::utils::config::app_config::AppConfig;
use crate::utils::ui::resources::AppStatus;
use rand::seq::SliceRandom;
use rand::thread_rng;

const SCIENTIST_QUOTES: &[(&str, &str)] = &[
    ("The important thing is not to stop questioning. Curiosity has its own reason for existence.", "Albert Einstein"),
    ("Science is organized knowledge. Wisdom is organized life.", "Immanuel Kant"),
    ("The only way to do great work is to love what you do.", "Steve Jobs"),
    ("Imagination is more important than knowledge.", "Albert Einstein"),
    ("Life is like riding a bicycle. To keep your balance, you must keep moving.", "Albert Einstein"),
    ("The science of today is the technology of tomorrow.", "Edward Teller"),
    ("An investment in knowledge pays the best interest.", "Benjamin Franklin"),
    ("Nothing in life is to be feared, it is only to be understood. Now is the time to understand more, so that we may fear less.", "Marie Curie"),
    ("The good thing about science is that it's true whether or not you believe in it.", "Neil deGrasse Tyson"),
    ("Somewhere, something incredible is waiting to be known.", "Carl Sagan"),
];

fn get_random_quote() -> (&'static str, &'static str) {
    let mut rng = thread_rng();
    *SCIENTIST_QUOTES.choose(&mut rng).unwrap_or(&("The universe is a pretty big place.", "Carl Sagan"))
}

pub struct StartMenuState {
    pub selected_index: Option<usize>,
    pub menu_items: Vec<(&'static str, &'static str, AppState)>,
    pub current_quote: String,
    pub current_author: String,
}

impl Default for StartMenuState {
    fn default() -> Self {
        let (quote, author) = get_random_quote();
        Self {
            selected_index: None,
            menu_items: vec![
                ("🗀", "File Explorer", AppState::FileExplorer),
                ("✎", "Text Editor", AppState::TextEditor),
                ("▶", "Emulator", AppState::Emulator),
                ("⚙", "Settings", AppState::Settings),
                ("🛈", "Help Guide", AppState::HelpGuide),
            ],
            current_quote: quote.to_string(),
            current_author: author.to_string(),
        }
    }
}

impl StartMenuState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn selected_item(&self) -> Option<&(&'static str, &'static str, AppState)> {
        self.selected_index.and_then(|i| self.menu_items.get(i))
    }
}

pub fn handle_start_menu_input(
    key: KeyEvent,
    menu_state: &mut StartMenuState,
    app_state: &mut AppState,
    app_status: &mut AppStatus,
    _workspace: &Workspace,
    handled: &mut bool,
) {
    if key.kind != KeyEventKind::Press {
        return;
    }

    let item_count = menu_state.menu_items.len();

    match key.code {
        KeyCode::Down => {
            menu_state.selected_index = Some(menu_state.selected_index.map_or(0, |i| (i + 1) % item_count));
            *handled = true;
        }
        KeyCode::Up => {
            menu_state.selected_index = Some(menu_state.selected_index.map_or(item_count - 1, |i| if i == 0 { item_count - 1 } else { i - 1 }));
            *handled = true;
        }
        KeyCode::Enter => {
            if let Some((_, _, target_state)) = menu_state.selected_item() {
                *app_state = target_state.clone();
                
                // Set a status message
                let message = match target_state {
                    AppState::FileExplorer => "Opening File Explorer...",
                    AppState::TextEditor => "Opening Text Editor...",
                    AppState::Emulator => "Opening Emulator...",
                    AppState::Settings => "Opening Settings...",
                    _ => "Navigating...",
                };
                app_status.set_message(message.to_string());
            }
            *handled = true;
        }
        KeyCode::Char(c) => {
            if ('1'..='5').contains(&c) {
                let index = (c as u8 - b'1') as usize;
                if index < item_count {
                    menu_state.selected_index = Some(index);
                    if let Some((_, _, target_state)) = menu_state.selected_item() {
                        *app_state = target_state.clone();
                    }
                }
            }
            *handled = true;
        }
        _ => {}
    }
}

// System to render the start menu
pub fn render_start_menu(
    frame: &mut Frame,
    area: Rect,
    menu_state: &StartMenuState,
    workspace: &Workspace,
    config: &AppConfig,
) {
    frame.render_widget(ratatui::widgets::Clear, area);
    let centered_area = centered_rect(80, 80, area);

    // Split the centered area into title, inspirational speech, menu, and workspace info
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(25), // Title
            Constraint::Percentage(10), // Inspirational Speech
            Constraint::Percentage(45),    // Menu
            Constraint::Percentage(20), // Workspace info
        ])
        .split(centered_area);

    // Render title
    render_title(frame, chunks[0]);

    // Render inspirational speech
    let quote_text = format!("\"{}\"\n- {}", menu_state.current_quote, menu_state.current_author);
    let quote_widget = Paragraph::new(Text::from(quote_text))
        .style(Style::default().fg(Color::Magenta))
        .alignment(ratatui::layout::Alignment::Center);
    frame.render_widget(quote_widget, chunks[1]);

    // Render menu
    render_menu(frame, chunks[2], menu_state);

    // Render workspace info
    render_workspace_info(frame, chunks[3], workspace, config);
}



fn render_title(frame: &mut Frame, area: Rect) {
    let title_str = r#"
   ▄▄▄         ██▀███      ▄████▄      ██████ 
▒████▄      ▓██ ▒ ██▒   ▒██▀ ▀█    ▒██    ▒ 
▒██  ▀█▄    ▓██ ░▄█ ▒   ▒▓█    ▄   ░ ▓██▄   
░██▄▄▄▄██   ▒██▀▀█▄     ▒▓▓▄ ▄██     ▒   ██▒
 ▓█   ▓████ ░██▓ ▒██▒██ ▒ ▓███▀ ██ ▒██████▒▒
 ▒▒   ▓▒█▒▒ ░ ▒▓ ░▒▓░▒▒ ░ ░▒ ▒  ▒▒ ▒ ▒▓▒ ▒ ░
  ░   ▒▒ ░    ░▒ ░ ▒░░    ░  ▒  ░  ░ ░▒  ░  
  ░   ▒  ░     ░   ░ ░  ░       ░  ░  ░  ░  
      ░   ░    ░      ░ ░ ░      ░       ░  
"#;

    let all_lines: Vec<&str> = title_str.lines().collect();
    let mut colored_lines: Vec<Line> = Vec::new();

    for line_content in all_lines.into_iter() {
        colored_lines.push(Line::from(Span::styled(line_content, Style::default().fg(Color::Cyan))));
    }

    let title_widget = Paragraph::new(Text::from(colored_lines))
        .alignment(ratatui::layout::Alignment::Center);

    frame.render_widget(title_widget, area);
}

fn render_menu(frame: &mut Frame, area: Rect, menu_state: &StartMenuState) {
    let items: Vec<ListItem> = menu_state.menu_items
        .iter()
        .enumerate()
        .map(|(index, (icon, name, _))| {
            let is_selected = menu_state.selected_index == Some(index);
            
            let style = if is_selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let content = Line::from(vec![
                icon.to_string().into(),
                "  ".into(),
                name.to_string().set_style(style),
                " ".into(),
                if is_selected { "◄".into() } else { " ".into() },
            ]);

            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(" Main Menu ")
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Double)
                .border_style(Style::default().fg(Color::LightBlue))
        )
        .highlight_symbol(">> ");

    frame.render_widget(list, area);
}

fn render_workspace_info(frame: &mut Frame, area: Rect, workspace: &Workspace, config: &AppConfig) {
    let open_files_count = workspace.open_files.len();
    let workspace_path = workspace.current_path.display();
    let theme = &config.ui.theme;
    let recent_files_limit = config.workspace.recent_files_limit;
    let auto_save_status = if config.workspace.auto_save { "On" } else { "Off" };

    let info_text = format!(
        "Workspace: {}\nLoaded files: {}\nRecent Files Limit: {}\nAuto Save: {}\nTheme: {}",
        workspace_path,
        open_files_count,
        recent_files_limit,
        auto_save_status,
        theme
    );

    let info_widget = Paragraph::new(Text::from(info_text))
        .block(
            Block::default()
                .title(" Workspace Status ")
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Double)
                .border_style(Style::default().fg(Color::LightGreen))
        )
        .style(Style::default().fg(Color::Gray));

    frame.render_widget(info_widget, area);
}

// System to initialize start menu state when entering the state
pub fn on_enter_start_menu(
    menu_state: &mut StartMenuState,
    app_status: &mut AppStatus,
) {
    // Reset selection when entering start menu
    menu_state.selected_index = None;
    
    // Update inspirational quote
    let (quote, author) = get_random_quote();
    menu_state.current_quote = quote.to_string();
    menu_state.current_author = author.to_string();
    
    // Set welcome message
    app_status.set_message("Welcome to ARC Emulator! Use arrow keys and Enter to navigate.".to_string());
}
