use crossterm::cursor::MoveTo;
use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers, poll, read};
use std::error::Error;
use std::process;
use std::process::{Command, Stdio};
use std::time::Duration;

use crossterm::terminal::{ClearType, disable_raw_mode};
use crossterm::{event::Event, execute};
use std::io::Write;
use std::io::stdout;

use crate::get_template_dir;

fn display_options(v: &Vec<String>, mut index: usize) -> Result<(), Box<dyn Error>> {
    index %= v.len();

    for i in 0..v.len() {
        if i == index {
            print!("> {}. {}\r\n", i, v[i]);
        } else {
            print!("{}. {}\r\n", i, v[i]);
        }
        stdout().flush()?;
    }
    Ok(())
}

fn clean_screen() -> Result<(), Box<dyn Error>> {
    execute!(
        stdout(),
        crossterm::terminal::Clear(ClearType::All),
        MoveTo(0, 0)
    )?;
    Ok(())
}

pub fn options(v: &Vec<String>) -> Result<usize, Box<dyn Error>> {
    let mut index = 0;
    clean_screen()?;
    display_options(v, index)?;
    loop {
        if let Event::Key(event) = read()? {
            if event.kind != KeyEventKind::Press {
                continue;
            }

            if event.code == KeyCode::Char('c') && 
            event.modifiers.contains(KeyModifiers::CONTROL) {
                    disable_raw_mode()?;
                    std::process::exit(0);
            }
            match event.code {
                KeyCode::Char('j') => {
                    index = (index + 1) % v.len();
                    clean_screen()?;
                    display_options(v, index)?;
                }
                KeyCode::Char('k') => {
                    if index == 0 {
                        index = v.len() - 1;
                    } else {
                        index -= 1;
                    }
                    clean_screen()?;
                    display_options(v, index)?;
                }

                KeyCode::Enter => {
                    clean_screen()?;
                    return Ok(index);
                }

                _ => {}
            }
        }
    }
}

pub fn options_template(v: &Vec<String>, extension: &String) -> Result<usize, Box<dyn Error>> {
    let template_dir = get_template_dir();
    let mut index = 0;
    clean_screen()?;
    display_options(v, index)?;
    loop {
        if let Event::Key(event) = read()? {
            if event.kind != KeyEventKind::Press {
                continue;
            }
            if event.code == KeyCode::Char('c') && 
            event.modifiers.contains(KeyModifiers::CONTROL) {
                    disable_raw_mode()?;
                    std::process::exit(0);
            }
            match event.code {
                KeyCode::Char('j') => {
                    index = (index + 1) % v.len();
                    clean_screen()?;
                    display_options(v, index)?;
                }
                KeyCode::Char('k') => {
                    if index == 0 {
                        index = v.len() - 1;
                    } else {
                        index -= 1;
                    }
                    clean_screen()?;
                    display_options(v, index)?;
                }

                KeyCode::Char('s') => {
                    let mut temp = template_dir.clone();
                    temp.push(format!("{}/{}.{}", extension, &v[index], extension));
                    let output = process::Command::new("bat").arg(&temp).spawn()?.wait()?;
                }
                KeyCode::Enter => {
                    clean_screen()?;
                    return Ok(index);
                }

                _ => {}
            }
        }
    }
}

pub fn input_prompt(prompt: &str) -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    print!("{}", prompt);
    stdout().flush()?;

    loop {
        if let Event::Key(event) = read()? {
            if event.kind != KeyEventKind::Press {
                continue;
            }

            if event.code == KeyCode::Char('c') && 
            event.modifiers.contains(KeyModifiers::CONTROL) {
                    disable_raw_mode()?;
                    std::process::exit(0);
            }

            match event.code {
                KeyCode::Char(c) => {
                    input.push(c);
                    print!("{}", c);
                    stdout().flush()?;
                }
                KeyCode::Backspace => {
                    if input.len() > 0 {
                        print!("\x08 \x08");
                        input.pop();
                    }
                    stdout().flush()?;
                }
                KeyCode::Enter => {
                    print!("\r\n");
                    stdout().flush()?;
                    break;
                }
                _ => {}
            }
        }
    }

    clean_screen()?;

    Ok(input)
}
