#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Command {
    Quit,
    Save,
    OpenFile,
    NewBuffer,
    SwitchBuffer,
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
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EditorAction {
    Move(Direction),
    Insert(char),
    Backspace,
    Run(Command),
    Execute(Macro),
    Unhandled(String),
}

pub trait LowerInput {
    fn get_event(&mut self) -> Option<EditorAction>;
}

pub struct Editor {
    pub line: usize,
    pub column: usize,
    pub content: String,
    pub running: bool,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            line: 0,
            column: 0,
            content: String::new(),
            running: true,
        }
    }

    pub fn eval(&mut self, action: EditorAction) -> () {
        use Command::*;
        use Direction::*;
        use EditorAction::*;
        use Macro::*;

        match action {
            Unhandled(_) => {}

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
            Backspace => {
                self.perform_backspace();
            }

            Run(Quit) => self.running = false,
            Run(Save) => {}
            Run(OpenFile) => {}
            Run(NewBuffer) => {}
            Run(Copy) => {}
            Run(Paste) => {}
            Run(Cut) => {}
            Run(SwitchBuffer) => {}

            Execute(SelectChar) => {}
            Execute(SelectLine) => {}
            Execute(SelectAll) => {}
            Execute(SelectUntilBufferEnd) => {}
            Execute(SelectUntilBufferStart) => {}
            Execute(SelectUntilLineEnd) => {}
            Execute(SelectUntilLineStart) => {}
        }
    }

    pub fn get_current_line(&self) -> Option<&str> {
        self.content.lines().nth(self.line)
    }

    fn snap_column(&mut self) {
        let len = self.get_current_line().map_or(0, |l| l.len());
        if self.column > len {
            self.column = len;
        }
    }

    fn can_move_up(&self) -> bool {
        self.line > 0
    }
    fn can_move_down(&self) -> bool {
        self.line < self.content.lines().count().saturating_sub(1)
    }
    fn can_move_right(&self) -> bool {
        self.get_current_line().map_or(0, |l| l.len()) > self.column
    }
    fn can_move_left(&self) -> bool {
        self.column > 0
    }
    fn can_backspace(&self) -> bool {
        self.line > 0 || self.column > 0
    }

    fn get_buffer_index(&self) -> usize {
        self.content
            .lines()
            .take(self.line)
            .map(|l| l.len() + 1)
            .sum::<usize>()
            + self.column
    }

    fn perform_backspace(&mut self) -> bool {
        let index = self.get_buffer_index();
        if self.can_backspace() {
            if index > 0 {
                self.content.remove(index - 1);
                if self.column > 0 {
                    self.column -= 1;
                } else {
                    self.line -= 1;
                    self.snap_column();
                }
            }
            return true;
        } else {
            return false;
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
        self.content.insert(idx, c);
        self.column += 1;
    }
}
