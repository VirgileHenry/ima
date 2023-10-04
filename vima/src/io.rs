pub struct IO {
    input: Vec<char>,
    /// length is at least one
    lines: Vec<String>,
    cursor_position: usize,
}

impl Default for IO {
    fn default() -> IO {
        IO {
            input: Vec::new(),
            lines: vec![String::new()],
            cursor_position: 0,
        }
    }
}

impl IO {
    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.input.len() {
            self.cursor_position += 1;
        }
    }

    pub fn enter_char(&mut self, new_char: char) {
        self.input.insert(self.cursor_position, new_char);
        self.move_cursor_right();
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.cursor_position != 0;
        if is_not_cursor_leftmost {
            self.input.remove(self.cursor_position - 1);
            self.move_cursor_left();
        }
    }

    pub fn reset_cursor(&mut self) {
        self.cursor_position = 0;
    }

    pub fn flush_input(&mut self) {
        let input: String = self.input.drain(..).collect();
        self.concat_line(input.as_str());
        self.cursor_position = 0;
    }

    pub fn new_line(&mut self) {
        self.lines.push(String::new());
    }

    pub fn concat_line(&mut self, line: &str) {
        for c in line.chars() {
            if c == '\n' {
                self.new_line();
            } else {
                let last = self.lines.len() - 1;
                self.lines[last].push(c);
            }
        }
    }

    pub fn cursor_pos(&self) -> usize {
        self.cursor_position + self.lines[self.lines.len() - 1].len()
    }

    pub fn input(&self) -> String {
        self.input.iter().map(|c| *c).collect()
    }

    pub fn display_stack(&self, height: usize) -> impl Iterator<Item = String> {
        let size_from_lines = self.lines.len().min(height - 1);
        if size_from_lines == 0 {
            return Vec::with_capacity(0).into_iter()
        }
        let mut iter = self.lines.as_slice()[self.lines.len() - size_from_lines..].iter().map(|s| s.clone()).collect::<Vec<_>>();
        let last = iter.len() - 1;
        iter[last].push_str(self.input().as_str());
        iter.into_iter()
    }
}

