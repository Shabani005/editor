mod editor;

use std::io::{self, Write, stdout};

use crossterm::{
    ExecutableCommand, QueueableCommand, cursor,
    event::{self, DisableBracketedPaste, EnableBracketedPaste, Event, KeyCode, KeyModifiers},
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};

use editor::{Command, Direction, Editor, EditorAction, LowerInput};

struct TerminalEditor;

impl LowerInput for TerminalEditor {
    fn get_event(&mut self) -> Option<EditorAction> {
        match event::read().ok()? {
            Event::Paste(txt) => Some(EditorAction::InsertString(txt)),

            Event::Key(key) => match key.code {
                KeyCode::Modifier(event::ModifierKeyCode::LeftControl) => {
                    Some(EditorAction::KeyDown(editor::Modifier::Ctrl))
                }
                // KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                //     Some(EditorAction::Run(Command::Quit))
                // }
                KeyCode::Up => Some(EditorAction::Move(Direction::Up)),
                KeyCode::Down => Some(EditorAction::Move(Direction::Down)),
                KeyCode::Left => Some(EditorAction::Move(Direction::Left)),
                KeyCode::Right => Some(EditorAction::Move(Direction::Right)),
                KeyCode::Backspace => Some(EditorAction::Backspace),
                KeyCode::Enter => Some(EditorAction::Insert('\n')),
                KeyCode::Char(c) => Some(EditorAction::Insert(c)),
                _ => Some(EditorAction::Unhandled(format!("{:?}", key.code))),
            },

            _ => None,
        }
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();

    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(EnableBracketedPaste)?;

    let mut my_editor = Editor::new();
    let mut keylifter = TerminalEditor;

    while my_editor.running {
        stdout.queue(cursor::MoveTo(0, 0))?;
        stdout.queue(Clear(ClearType::All))?;

        print!(
            "EDITOR | Line: {}, Col: {} | Ctrl+Q to Quit\r\n",
            my_editor.line, my_editor.column
        ); // Line 1
        print!("-------------------------------------------\r\n"); // Line 2

        let line_count = my_editor.content().len_lines().max(1);

        for i in 0..line_count {
            let line = my_editor.content().line(i);
            let text = line.to_string().trim_end_matches('\n').to_string();
            print!("{}\r\n", text);
        }

        stdout.queue(cursor::MoveTo(
            my_editor.column as u16,
            my_editor.line as u16 + 2,
        ))?;

        stdout.flush()?;

        if let Some(action) = keylifter.get_event() {
            my_editor.eval(action);
        }
    }

    stdout.execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    stdout.execute(DisableBracketedPaste)?;
    Ok(())
}
