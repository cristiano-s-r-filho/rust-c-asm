use ratatui::{
    widgets::{Block, Borders, Paragraph, Wrap},
    text::{Text, Line, Span},
    style::{Style, Color, Modifier},
    layout::{Rect},
    Frame,
};
use crossterm::event::{KeyEvent, KeyCode, KeyEventKind};

use crate::utils::ui::common::AppState;
use crate::utils::apps::help_content::get_help_sections;

pub struct HelpGuideState {
    pub selected_section: usize,
    pub scroll_offset: usize,
    pub sections: Vec<HelpSection>,
}

pub struct HelpSection {
    pub title: String,
    pub content: Vec<String>,
}

impl Default for HelpGuideState {
    fn default() -> Self {
        Self {
            selected_section: 0,
            scroll_offset: 0,
            sections: get_help_sections(),
        }
    }
}

fn parse_content(content: &[String]) -> Text<'_> {
    let mut text = Text::default();
    for line_str in content {
        let mut line = Line::default();
        let mut parts = line_str.split("`").collect::<Vec<_>>();
        for (i, part) in parts.iter_mut().enumerate() {
            if i % 2 == 1 {
                line.spans.push(Span::styled(*part, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)));
            } else {
                line.spans.push(Span::raw(*part));
            }
        }
        text.lines.push(line);
    }
    text
}

pub fn render_help_guide(
    frame: &mut Frame,
    area: Rect,
    help_guide_state: &mut HelpGuideState,
) {
    let current_section = &help_guide_state.sections[help_guide_state.selected_section];
    let content_height = current_section.content.len();

    let text = parse_content(&current_section.content);

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title(current_section.title.clone()))
        .wrap(Wrap { trim: true })
        .scroll((help_guide_state.scroll_offset as u16, 0));

    frame.render_widget(paragraph, area);

    // Clamp scroll_offset
    if help_guide_state.scroll_offset >= content_height.saturating_sub(area.height as usize) {
        help_guide_state.scroll_offset = content_height.saturating_sub(area.height as usize);
    }
}

pub fn handle_help_guide_input(
    key: KeyEvent,
    help_guide_state: &mut HelpGuideState,
    app_state: &mut AppState,
    handled: &mut bool,
) {
    if key.kind != KeyEventKind::Press {
        return;
    }

    match key.code {
        KeyCode::Esc => {
            *app_state = AppState::StartMenu;
            *handled = true;
        },
        KeyCode::Right => {
            if help_guide_state.selected_section < help_guide_state.sections.len() - 1 {
                help_guide_state.selected_section += 1;
                help_guide_state.scroll_offset = 0;
            }
            *handled = true;
        },
        KeyCode::Left => {
            if help_guide_state.selected_section > 0 {
                help_guide_state.selected_section -= 1;
                help_guide_state.scroll_offset = 0;
            }
            *handled = true;
        },
        KeyCode::Down => {
            help_guide_state.scroll_offset = help_guide_state.scroll_offset.saturating_add(1);
            *handled = true;
        },
        KeyCode::Up => {
            help_guide_state.scroll_offset = help_guide_state.scroll_offset.saturating_sub(1);
            *handled = true;
        },
        _ => {}
    }
}
