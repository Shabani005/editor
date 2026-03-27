mod editor;

use std::io::{self, Write, stdout};

use crossterm::{
    ExecutableCommand, QueueableCommand, cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};

use editor::{Command, Direction, Editor, EditorAction, LowerInput};

struct TerminalEditor;

impl LowerInput for TerminalEditor {
    fn get_event(&mut self) -> Option<EditorAction> {
        if let Ok(Event::Key(key)) = event::read() {
            return match key.code {
                KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    Some(EditorAction::Run(Command::Quit))
                }
                KeyCode::Up => Some(EditorAction::Move(Direction::Up)),
                KeyCode::Down => Some(EditorAction::Move(Direction::Down)),
                KeyCode::Left => Some(EditorAction::Move(Direction::Left)),
                KeyCode::Right => Some(EditorAction::Move(Direction::Right)),
                KeyCode::Backspace => Some(EditorAction::Backspace),
                KeyCode::Enter => Some(EditorAction::Insert('\n')),
                KeyCode::Char(c) => Some(EditorAction::Insert(c)),
                _ => Some(EditorAction::Unhandled(format!("{:?}", key.code))),
            };
        }
        None
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    stdout.execute(EnterAlternateScreen)?;

    let mut my_editor = Editor::new();
    let mut keylifter = TerminalEditor;

    while my_editor.running {
        stdout.queue(cursor::MoveTo(0, 0))?;
        stdout.queue(Clear(ClearType::All))?;

        print!(
            "EDITOR | Line: {}, Col: {} | Ctrl+Q to Quit\r\n",
            my_editor.line, my_editor.column
        );
        print!("-------------------------------------------\r\n");

        let lines: Vec<&str> = my_editor.content.lines().collect();
        let max_lines = lines.len().max(1);

        for i in 0..max_lines {
            let text = lines.get(i).unwrap_or(&"");
            print!("{}\r\n", text);
        }

        stdout.flush()?;

        if let Some(action) = keylifter.get_event() {
            my_editor.eval(action);
        }
    }

    stdout.execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
