use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self, Write};

const BOX_BORDER: &str = "===========================================================================================================================";

fn clear_screen() {
    print!("\x1B[3J\x1B[2J\x1B[1;1H");
    let _ = io::stdout().flush();
}

fn print_banner() {
    println!("{}", BOX_BORDER);
    println!(" 🏠 HOMECALC - HOUSING FINANCIAL SUITE");
    println!("{}", BOX_BORDER);
}

fn select_main_menu(options: &[&str]) -> Result<usize, io::Error> {
    let mut selected_idx: usize = 0;
    let total = options.len();

    if total == 0 {
        return Ok(0);
    }

    loop {
        clear_screen();
        print_banner();

        println!();
        for (idx, opt) in options.iter().enumerate() {
            let num = idx + 1;
            if idx == selected_idx {
                println!("  \x1B[1;33m➔ [{}] {}\x1B[0m", num, opt);
            } else {
                println!("    \x1B[2m[{}] {}\x1B[0m", num, opt);
            }
        }
        println!();
        let _ = io::stdout().flush();

        enable_raw_mode()?;
        let event_res = event::read();
        let _ = disable_raw_mode();

        match event_res {
            Ok(Event::Key(KeyEvent { code, modifiers, .. })) => {
                if modifiers.contains(KeyModifiers::CONTROL) && code == KeyCode::Char('c') {
                    return Ok(total.saturating_sub(1));
                }

                match code {
                    KeyCode::Up => {
                        selected_idx = (selected_idx + total - 1) % total;
                    }
                    KeyCode::Down => {
                        selected_idx = (selected_idx + 1) % total;
                    }
                    KeyCode::Enter => {
                        return Ok(selected_idx);
                    }
                    KeyCode::Char(ch) => {
                        if let Some(digit) = ch.to_digit(10) {
                            let d = digit as usize;
                            if d >= 1 && d <= total {
                                return Ok(d - 1);
                            }
                        }
                        if ch == 'q' || ch == 'Q' {
                            return Ok(total.saturating_sub(1));
                        }
                    }
                    KeyCode::Esc => {
                        return Ok(total.saturating_sub(1));
                    }
                    _ => {}
                }
            }
            Err(e) => {
                return Err(e);
            }
            _ => {}
        }
    }
}

fn main() {
    let options = [
        "Amortizing Mortgage Calculator",
        "Portfolio Line of Credit (SBLOC) Calculator",
        "Exit",
    ];

    loop {
        match select_main_menu(&options) {
            Ok(0) => {
                mortgage::ui::cli::run_cli();
            }
            Ok(1) => {
                loc::ui::cli::run_cli();
            }
            Ok(2) | Ok(_) => {
                clear_screen();
                println!("Thank you for using Homecalc! Goodbye.");
                break;
            }
            Err(_) => {
                clear_screen();
                println!("Thank you for using Homecalc! Goodbye.");
                break;
            }
        }
    }
}
