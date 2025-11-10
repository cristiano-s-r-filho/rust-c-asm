//! # Symbols Module
//!
//! This module defines various constant string symbols used for rendering
//! UI elements in the terminal. These symbols include box drawing characters,
//! blocks, arrows, and other special characters.

// Box drawing - Light
pub const LINE_HORIZONTAL: &str = "─";
pub const LINE_VERTICAL: &str = "│";
pub const CORNER_TOP_LEFT: &str = "┌";
pub const CORNER_TOP_RIGHT: &str = "┐";
pub const CORNER_BOTTOM_LEFT: &str = "└";
pub const CORNER_BOTTOM_RIGHT: &str = "┘";
pub const T_RIGHT: &str = "├";
pub const T_LEFT: &str = "┤";
pub const T_DOWN: &str = "┬";
pub const T_UP: &str = "┴";
pub const CROSS_LINES: &str = "┼";

// Box drawing - Double
pub const LINE_HORIZONTAL_DOUBLE: &str = "═";
pub const LINE_VERTICAL_DOUBLE: &str = "║";
pub const CORNER_TOP_LEFT_DOUBLE: &str = "╔";
pub const CORNER_TOP_RIGHT_DOUBLE: &str = "╗";
pub const CORNER_BOTTOM_LEFT_DOUBLE: &str = "╚";
pub const CORNER_BOTTOM_RIGHT_DOUBLE: &str = "╝";

// Blocks and shapes
pub const BLOCK_FULL: &str = "█";
pub const BLOCK_DARK: &str = "▓";
pub const BLOCK_MEDIUM: &str = "▒";
pub const BLOCK_LIGHT: &str = "░";
pub const SQUARE_FILLED: &str = "■";
pub const SQUARE_EMPTY: &str = "□";
pub const SQUARE_SMALL: &str = "▪";

// Arrows
pub const ARROW_LEFT: &str = "←";
pub const ARROW_RIGHT: &str = "→";
pub const ARROW_UP: &str = "↑";
pub const ARROW_DOWN: &str = "↓";
pub const ARROW_DOUBLE_LEFT: &str = "⇐";
pub const ARROW_DOUBLE_RIGHT: &str = "⇒";
pub const ARROW_DOUBLE_UP: &str = "⇑";
pub const ARROW_DOUBLE_DOWN: &str = "⇓";
pub const POINTER_RIGHT: &str = "►";
pub const POINTER_LEFT: &str = "◄";

// Crosses and checks
pub const CROSS: &str = "✕";
pub const CROSS_HEAVY: &str = "✖";
pub const CHECK: &str = "✓";
pub const CHECK_HEAVY: &str = "✔";

// Circles and dots
pub const CIRCLE_FILLED: &str = "●";
pub const CIRCLE_EMPTY: &str = "○";
pub const BULLET: &str = "•";

// UI elements
pub const HOME: &str = "⌂";
pub const COMMAND_KEY: &str = "⌘";
pub const BACKSPACE: &str = "⌫";
pub const DELETE: &str = "⌦";
pub const RETURN: &str = "⏎";
pub const POWER: &str = "⏻";

// Special symbols
pub const POINTER_HEAVY_RIGHT: &str = "❯"; 
pub const WARNING: &str = "⚠";
pub const STAR_FILLED: &str = "★";
pub const RECYCLING: &str = "♻";
pub const HIGH_VOLTAGE: &str = "⚡";