// src/main.rs
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use std::collections::HashMap;
use std::io::{stdout, Write};
use std::io;
use std::time::{Duration, Instant};

fn morse_map() -> HashMap<char, &'static str> {
    [
        ('A', ".-"),    ('B', "-..."),  ('C', "-.-."),  ('D', "-.."),
        ('E', "."),     ('F', "..-."),  ('G', "--."),   ('H', "...."),
        ('I', ".."),    ('J', ".---"),  ('K', "-.-"),   ('L', ".-.."),
        ('M', "--"),    ('N', "-."),    ('O', "---"),   ('P', ".--."),
        ('Q', "--.-"),  ('R', ".-."),   ('S', "..."),   ('T', "-"),
        ('U', "..-"),   ('V', "...-"),  ('W', ".--"),   ('X', "-..-"),
        ('Y', "-.--"),  ('Z', "--.."),  (' ', " "),
    ].iter().cloned().collect()
}

fn get_expected_morse(text: &str, map: &HashMap<char, &str>) -> String {
    text.to_uppercase()
        .chars()
        .filter_map(|c| map.get(&c).cloned())
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() -> io::Result<()> {
    let target_text = "HELLO WORLD";
    let morse_dict = morse_map();
    let expected_morse = get_expected_morse(target_text, &morse_dict);

    println!("Type the following in Morse Code using [Space] key:");
    println!("{}", target_text);
    println!("- dot = quick press, dash = long press (>300ms)");
    println!("Press [Esc] to finish early.\n");

    let mut input = String::new();
    let mut press_start: Option<Instant> = None;
    let mut keypress_count = 0;
    let start_time = Instant::now();

    loop {
        if let Event::Key(KeyEvent { code, modifiers: _, kind: _, state: _ }) = read()? {
            match code {
                KeyCode::Esc => break,
                KeyCode::Char(' ') => {
                    let now = Instant::now();
                    match press_start {
                        None => {
                            press_start = Some(now);
                        }
                        Some(start) => {
                            let duration = now - start;
                            if duration < Duration::from_millis(300) {
                                input.push('.');
                                print!(".");
                            } else {
                                input.push('-');
                                print!("-");
                            }
                            stdout().flush().unwrap();
                            press_start = None;
                            keypress_count += 1;
                        }
                    }
                }
                KeyCode::Enter => {
                    input.push(' ');
                    print!(" ");
                    stdout().flush().unwrap();
                }
                _ => {}
            }
        }

        if input.trim() == expected_morse.trim() {
            break;
        }
    }

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
