use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::collections::HashMap;
use std::io::{stdout, Write};
use std::time::{Duration, Instant};

fn morse_map() -> HashMap<char, &'static str> {
    [
        ('A', ".-"),    ('B', "-..."),  ('C', "-.-."),  ('D', "-.."),
        ('E', "."),     ('F', "..-."),  ('G', "--."),   ('H', "...."),
        ('I', ".."),    ('J', ".---"),  ('K', "-.-"),   ('L', ".-.."),
        ('M', "--"),    ('N', "-."),    ('O', "---"),   ('P', ".--."),
        ('Q', "--.-"),  ('R', ".-."),   ('S', "..."),   ('T', "-"),
        ('U', "..-"),   ('V', "...-"),  ('W', ".--"),   ('X', "-..-"),
        ('Y', "-.--"),  ('Z', "--.."),  (' ', "/"),
    ].iter().cloned().collect()
}

fn get_expected_morse(text: &str, map: &HashMap<char, &str>) -> String {
    text.to_uppercase()
        .chars()
        .filter_map(|c| map.get(&c).cloned())
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target_text = "HELLO WORLD";
    let morse_dict = morse_map();
    let expected_morse = get_expected_morse(target_text, &morse_dict);

    println!("Type the following in Morse Code using [Space] key:");
    println!("{}", target_text);
    println!("- dot = quick press, dash = long press (>300ms)");
    println!("- Press [Enter] for space between letters");
    println!("- Press [Tab] for space between words");
    println!("Press [Esc] to finish early.\n");

    let mut stdout = stdout();
    enable_raw_mode()?;

    let mut input = String::new();
    let mut current_symbol = String::new();
    let mut keypress_count = 0;
    let start_time = Instant::now();

    // Track space press state
    let mut space_pressed = false;
    let mut press_start: Option<Instant> = None;
    let mut displayed_symbol = ' ';
    let mut symbol_finalized = false;

    print!("Your Input: ");
    stdout.flush()?;

    'main_loop: loop {
        // Check for events with timeout
        if poll(Duration::from_millis(10))? {
            match read()? {
                Event::Key(KeyEvent { code, kind, .. }) => {
                    if kind == KeyEventKind::Repeat {
                        continue;
                    }
                    match (code, kind) {
                        (KeyCode::Char(' ')  , KeyEventKind::Press) => {
                            // Spacja wciśnięta
                            if !space_pressed {
                                space_pressed = true;
                                press_start = Some(Instant::now());
                                displayed_symbol = '.';
                                symbol_finalized = false;
                                print!(".");
                                stdout.flush()?;
                            }
                        }
                        (KeyCode::Char(' '), KeyEventKind::Release) => {
                            // Spacja puszczona
                            if space_pressed {
                                let symbol = if press_start.unwrap().elapsed() < Duration::from_millis(300) {
                                    '.' // krótki nacisk
                                } else {
                                    '-' // długi nacisk
                                };

                                if displayed_symbol != symbol {
                                    execute!(stdout, cursor::MoveLeft(1))?;
                                    print!("{}", symbol);
                                    stdout.flush()?;
                                    displayed_symbol = symbol;
                                }

                                current_symbol.push(symbol);
                                keypress_count += 1;
                                space_pressed = false;
                                symbol_finalized = true;
                            }
                        }
                        (KeyCode::Enter, KeyEventKind::Press) => {
                            if !current_symbol.is_empty() {
                                input.push_str(&current_symbol);
                                input.push(' ');
                                current_symbol.clear();
                                print!(" ");
                                stdout.flush()?;
                            }
                        }
                        (KeyCode::Tab, KeyEventKind::Press) => {
                            if !current_symbol.is_empty() {
                                input.push_str(&current_symbol);
                                input.push(' ');
                                current_symbol.clear();
                            }
                            input.push('/');
                            input.push(' ');
                            print!("/ ");
                            stdout.flush()?;
                        }
                        (KeyCode::Esc, _) => break 'main_loop,
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        // Aktualizuj symbol podczas trzymania spacji, tylko jeśli symbol nie został jeszcze zfinalizowany
        if space_pressed && !symbol_finalized {
            if let Some(start) = press_start {
                if start.elapsed() >= Duration::from_millis(300) && displayed_symbol == '.' {
                    execute!(stdout, cursor::MoveLeft(1))?;
                    print!("-");
                    stdout.flush()?;
                    displayed_symbol = '-';
                }
            }
        }

        // Sprawdź czy zakończono wpisywanie
        let input_with_current = if current_symbol.is_empty() {
            input.trim().to_string()
        } else {
            format!("{} {}", input.trim(), current_symbol)
        };

        if input_with_current == expected_morse {
            break;
        }
    }

    // Dodaj pozostały symbol
    if !current_symbol.is_empty() {
        input.push_str(&current_symbol);
    }

    disable_raw_mode()?;

    let elapsed = Instant::now() - start_time;
    println!("\n\nExpected Morse: {}", expected_morse);
    println!("Your Input:     {}", input.trim());
    println!("Time: {:.2?}", elapsed);
    println!("Keypresses: {}", keypress_count);
    println!(
        "Speed: {:.2} symbols/sec",
        input.len() as f64 / elapsed.as_secs_f64()
    );

    Ok(())
}
