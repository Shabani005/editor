use ropey::Rope;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Modifier {
    Ctrl,
    Alt,
    Shift,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Command {
    Quit,
    Save(String),
    OpenFile(String),
    NewBuffer,
    SwitchBuffer(usize),
    Copy,
    Cut,
    Paste,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Macro {
    SelectChar,
    SelectLine,
    SelectAll,
    SelectUntilBufferEnd,
    SelectUntilBufferStart,
    SelectUntilLineEnd,
    SelectUntilLineStart,
    MoveUntilLineStart,
    MoveUntilLineEnd,
    MoveUntilBufferStart,
    MoveUntilBufferEnd,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EditorAction {
    Move(Direction),
    Insert(char),
    InsertString(String),
    Backspace,
    Run(Command),   // TODO
    Execute(Macro), // TODO
    KeyUp(Modifier),
    KeyDown(Modifier),
    Unhandled(String),
}

pub trait LowerInput {
    fn get_event(&mut self) -> Option<EditorAction>;
}

pub struct Editor {
    pub line: usize,
    pub column: usize,
    pub contents: Vec<Rope>,
    pub running: bool,
    pub ctrl_held: bool,
    pub commands: Vec<char>,
    pub sel_start: Option<usize>,
    pub sel_end: Option<usize>,
    pub clipboard: String,
    pub current_buffer: usize,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            line: 0,
            column: 0,
            contents: vec![Rope::new()],
            commands: Vec::new(),
            sel_start: None,
            sel_end: None,
            clipboard: String::new(),
            running: true,
            ctrl_held: false,
            current_buffer: 0,
        }
    }

    pub fn eval(&mut self, action: EditorAction) -> () {
        use Command::*;
        use Direction::*;
        use EditorAction::*;
        use Macro::*;
        use Modifier::*;

        match action {
            Unhandled(_) => {}

            KeyDown(Ctrl) => {
                self.ctrl_held = true;
                self.commands.clear();
            }

            KeyUp(Ctrl) => {
                self.ctrl_held = false;
                self.run_command();
            }

            KeyDown(key) => {}
            KeyUp(key) => {}

            action if self.ctrl_held => {
                if let EditorAction::Insert(c) = action {
                    self.commands.push(c);
                }
            }

            Move(Up) => {
                self.move_up();
            }
            Move(Down) => {
                self.move_down();
            }
            Move(Right) => {
                self.move_right();
            }
            Move(Left) => {
                self.move_left();
            }
            Insert(c) => {
                self.insert_char(c);
            }
            InsertString(s) => {
                self.insert_string(s);
            }
            Backspace => {
                self.perform_backspace();
            }

            Run(Quit) => self.running = false,

            Run(Save(path)) => {
                self.write_content_to_file(path);
            }

            Run(OpenFile(path)) => {
                self.set_content_from_file(path);
            }

            Run(NewBuffer) => {}
            Run(Copy) => {}
            Run(Paste) => {}
            Run(Cut) => {}
            Run(SwitchBuffer(id)) => {}

            Execute(SelectChar) => {}
            Execute(SelectLine) => {}
            Execute(SelectAll) => {}

            Execute(SelectUntilBufferEnd) => {}
            Execute(SelectUntilBufferStart) => {}
            Execute(SelectUntilLineEnd) => {}
            Execute(SelectUntilLineStart) => {}

            Execute(MoveUntilLineStart) => {}
            Execute(MoveUntilLineEnd) => {}
            Execute(MoveUntilBufferStart) => {}
            Execute(MoveUntilBufferEnd) => {}
        }
    }

    pub fn get_current_line_len(&self) -> usize {
        self.content_immutable().line(self.line).len_chars()
    }

    fn snap_column(&mut self) {
        let len = self.get_current_line_len();
        if self.column > len {
            self.column = len;
        }
    }

    fn can_move_up(&self) -> bool {
        self.line > 0
    }
    fn can_move_down(&self) -> bool {
        self.line < self.content_immutable().lines().count().saturating_sub(1)
    }
    fn can_move_right(&self) -> bool {
        self.get_current_line_len() > self.column
    }
    fn can_move_left(&self) -> bool {
        self.column > 0
    }
    fn can_backspace(&self) -> bool {
        self.line > 0 || self.column > 0
    }

    fn get_buffer_index(&self) -> usize {
        self.content_immutable().line_to_char(self.line) + self.column
    }

    fn perform_backspace(&mut self) -> bool {
        let index = self.get_buffer_index();

        if !self.can_backspace() {
            return false;
        } else {
            if self.can_backspace() {
                if index > 0 {
                    self.content().remove((index - 1)..index);
                    if self.column > 0 {
                        self.column -= 1;
                    } else {
                        self.line -= 1;
                        self.snap_column();
                    }
                }
                return true;
            } else {
                false
            }
        }
    }

    fn move_up(&mut self) -> bool {
        if self.can_move_up() {
            self.line -= 1;
            self.snap_column();
            return true;
        } else {
            return false;
        }
    }

    fn move_down(&mut self) -> bool {
        if self.can_move_down() {
            self.line += 1;
            self.snap_column();
            return true;
        } else {
            return false;
        }
    }

    fn move_right(&mut self) -> bool {
        if self.can_move_right() {
            self.column += 1;
            return true;
        } else {
            return false;
        }
    }

    fn move_left(&mut self) -> bool {
        if self.can_move_left() {
            self.column -= 1;
            return true;
        } else {
            return false;
        }
    }

    fn insert_char(&mut self, c: char) {
        let idx = self.get_buffer_index();
        self.content().insert_char(idx, c);

        if c == '\n' {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }
    }

    fn insert_string(&mut self, s: String) {
        let idx = self.get_buffer_index();
        self.content().insert(idx, s.as_str());

        let newlines = s.chars().filter(|&c| c == '\n').count();
        if newlines > 0 {
            self.line += newlines;
            self.column = s.rfind('\n').map_or(0, |i| s[i + 1..].len());
        } else {
            self.column += s.chars().count();
        }
    }

    fn set_content_from_file(&mut self, path: String) {
        *self.content() = Rope::from_str(self.read_file(path.as_str()).as_str());
    }

    fn write_content_to_file(&mut self, path: String) {
        self.write_file(path.as_str());
    }

    fn read_file(&mut self, path: &str) -> String {
        fs::read_to_string(path).unwrap()
    }

    fn write_file(&mut self, path: &str) -> () {
        fs::write(path, self.content().to_string()).unwrap()
    }

    pub fn content(&mut self) -> &mut Rope {
        &mut self.contents[self.current_buffer]
    }

    pub fn content_immutable(&self) -> &Rope {
        &self.contents[self.current_buffer]
    }

    fn new_buffer(&mut self) {
        self.contents.push(Rope::new());
        self.current_buffer = self.contents.len() - 1;
        self.line = 0;
        self.column = 0;
    }

    fn switch_buffer(&mut self, id: usize) {
        if id < self.contents.len() {
            self.current_buffer = id;
            self.line = 0;
            self.column = 0;
        }
    }

    fn run_command(&mut self) {
        use Command::*;
        use EditorAction::*;

        let cmd: String = self.commands.iter().collect();

        match cmd.as_str() {
            "s" => self.eval(Run(Save("test.txt".to_string()))),
            "q" => self.eval(Run(Quit)),

            _ => {}
        }
    }
}
