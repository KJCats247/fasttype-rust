use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::event::{read, Event, KeyCode, KeyEventKind};
use ratatui::Terminal;
use ratatui::prelude::CrosstermBackend;
use ratatui::widgets::Paragraph;
use ratatui::text::{Line, Span};
use ratatui::style::{Style, Color};
use ratatui::layout::{Layout, Constraint, Direction};
use std::fs; // NEU: Wir brauchen fs für file system Zugriff

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filepath: String = std::env::args().nth(1).expect("No Filepath given!");
    let text = std::fs::read_to_string(&filepath).expect("Invalid File!");

    // NEU 1: Pfad für die Speicherdatei definieren
    let save_path = format!("{}.save", filepath);

    // NEU 2: Versuchen, den Spielstand zu laden
    // Wenn die Datei existiert, nehmen wir ihren Inhalt. Wenn nicht, starten wir leer.
    let mut user_input = fs::read_to_string(&save_path).unwrap_or_else(|_| String::new());

    enable_raw_mode().expect("Could not enable raw mode");

    let mut stdout = std::io::stdout();
    crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen)
        .expect("Couldn't enter alternate screen");

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|frame| {
            let current_line_index = user_input.chars().filter(|c| *c == '\n').count();
            let written_chars = user_input.split('\n').last().unwrap_or("").chars().count() as u16;
            
            let mut current_spans = Vec::new();
            let mut lines = Vec::new();
            let input_len = user_input.len();
            
            for (i, char_original) in text.chars().enumerate() {
                if char_original == '\n' {
                    lines.push(Line::from(current_spans));
                    current_spans = Vec::new();
                    continue;
                }
                
                if i < input_len {
                    let user_input_char = user_input.chars().nth(i).unwrap_or('\0');
                    if user_input_char == char_original {
                        current_spans.push(Span::styled(user_input_char.to_string(), Style::default().fg(Color::Green)));
                    } else {
                        current_spans.push(Span::styled(user_input_char.to_string(), Style::default().fg(Color::Red)));
                    }
                } else {
                    current_spans.push(Span::styled(char_original.to_string(), Style::default().fg(Color::DarkGray)));
                }
            }
            lines.push(Line::from(current_spans));

            let vertical_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Percentage(25),
                    Constraint::Length(1), 
                    Constraint::Length(1),
                    Constraint::Length(1), 
                    Constraint::Fill(1),
                ])
                .split(frame.size());

            if current_line_index < lines.len() {
                let past_lines = if current_line_index > 0 {
                    lines[0..current_line_index].to_vec()
                } else {
                    vec![]
                };
                
                let past_height = vertical_layout[0].height as usize;
                let past_scroll = if past_lines.len() > past_height {
                    (past_lines.len() - past_height) as u16
                } else {
                    0
                };
                
                let p_past = Paragraph::new(past_lines).scroll((past_scroll, 0));
                frame.render_widget(p_past, vertical_layout[0]);

                let horizontal_layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(vec![
                        Constraint::Min(0),
                        Constraint::Percentage(50),
                        Constraint::Min(0),
                    ])
                    .split(vertical_layout[2]);

                let scroll_amount = written_chars.saturating_sub(horizontal_layout[1].width / 2);
                let p_current = Paragraph::new(lines[current_line_index].clone())
                    .scroll((0, scroll_amount));
                
                frame.render_widget(p_current, horizontal_layout[1]);

                let future_lines = if current_line_index + 1 < lines.len() {
                    lines[(current_line_index + 1)..].to_vec()
                } else {
                    vec![]
                };
                let p_future = Paragraph::new(future_lines);
                frame.render_widget(p_future, vertical_layout[4]);

            } else {
                let p_done = Paragraph::new("Congratulations. You are done.")
                    .alignment(ratatui::layout::Alignment::Center);
                frame.render_widget(p_done, vertical_layout[2]);
                
                // Optional: Wenn man fertig ist, Speicherdatei löschen?
                // fs::remove_file(&save_path).ok(); 
            }

        })?;

        match read() {
            Ok(Event::Key(key)) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Esc => {
                    // NEU 3: Speichern beim Beenden
                    // Wir schreiben alles, was in user_input ist, in die Datei.
                    fs::write(&save_path, &user_input).expect("Could not save progress!");
                    break;
                },
                KeyCode::Char(c) => user_input.push(c),
                KeyCode::Backspace => { user_input.pop(); },
                KeyCode::Enter => user_input.push('\n'),
                _ => {}
            },
            _ => {}
        }
    }

    crossterm::execute!(terminal.backend_mut(), crossterm::terminal::LeaveAlternateScreen)
        .expect("Couldn't leave alternate screen");
    disable_raw_mode().expect("Could not disable raw mode");
    Ok(())
}
