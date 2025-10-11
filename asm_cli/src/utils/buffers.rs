use std::io::{
    Result,
    Write
}; 

// Basic color definitions.: 
pub const BLACK: &str = "\x1b[30m";
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[34m";
pub const MAGENTA: &str = "\x1b[35m";
pub const CYAN: &str = "\x1b[36m";
pub const WHITE: &str = "\x1b[37m"; 
// Background colors Definitions.: 
pub const BG_BLACK: &str = "\x1b[40m";
pub const BG_RED: &str = "\x1b[41m";
pub const BG_GREEN: &str = "\x1b[42m";
pub const BG_YELLOW: &str = "\x1b[43m";
pub const BG_BLUE: &str = "\x1b[44m";
pub const BG_MAGENTA: &str = "\x1b[45m";
pub const BG_CYAN: &str = "\x1b[46m";
pub const BG_WHITE: &str = "\x1b[47m";
// Style definitions.: 
pub const BOLD: &str = "\x1b[1m";
pub const DIM: &str = "\x1b[2m";
pub const ITALIC: &str = "\x1b[3m";
pub const UNDERLINED: &str = "\x1b[4m";
// RESET SETTING.: 
pub const RESET: &str = "\x1b[0m";

pub struct DisplayLineWriter<W:Write> {
    inner:W, 
    buff: Vec<u8>, 
    use_color: bool, 
    use_format: bool, 
} 

impl<W:Write> DisplayLineWriter<W> {
    pub fn new(bond: W, allow_color: bool, allow_format: bool) -> Self{
        DisplayLineWriter { 
            inner: bond, 
            buff: Vec::with_capacity(1024), 
            use_color: allow_color, 
            use_format: allow_format,  
        }
    }
    /// Auxiliary ANSI function for the write. Doens't need to be public, since it is only an auxiliary function
    /// It does not make itself available outside here. 
    fn write_ansi(&mut self, code: &str) -> Result<()> {
        if self.use_color || self.use_format {
            self.buff.extend_from_slice(code.as_bytes());
        }
        Ok(())
    }
    /// Set a combination of Color and Format. 
    pub fn set_style(&mut self, color: &str, format: Option<&str>) -> Result<()> {
        self.write_ansi(color)?;
        if let Some(s) = format {
            self.write_ansi(s)?;
        }
        Ok(())
    }
    pub fn set_background(&mut self, _background: &str) -> Result<()> {
        Ok(())
    }
    pub fn reset_style(&mut self) -> Result<()> {
        self.write_ansi(RESET)
    }

    pub fn flush_line(&mut self) -> Result<()> {
        if !self.buff.is_empty() {
            self.inner.write_all(&self.buff)?;
            self.inner.flush()?;
            self.buff.clear();
        }
        // Always reset after flushing to prevent style bleed
        self.write_ansi(RESET)?;
        Ok(())
    }
}

impl<W:Write> Write for DisplayLineWriter<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let mut written = 0;
        let mut split_iter = buf.split_inclusive(|&b| b == b'\n');

        while let Some(part) = split_iter.next() {
            self.buff.extend_from_slice(part);
        
            if part.ends_with(b"\n") {
                self.flush_line()?;
            }
            written += part.len();
        }
        Ok(written)
    }

    fn flush(&mut self) -> Result<()> {
        self.flush_line()
    }
}