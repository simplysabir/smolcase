use colored::*;
use dialoguer::{Confirm, Input, Password, Select};
use indicatif::{ProgressBar, ProgressStyle};
use std::io::{self, Write};

pub struct UI;

impl UI {
    pub fn success(msg: &str) {
        println!("{} {}", "✓".green().bold(), msg);
    }

    pub fn error(msg: &str) {
        eprintln!("{} {}", "✗".red().bold(), msg);
    }

    pub fn info(msg: &str) {
        println!("{} {}", "ℹ".blue().bold(), msg);
    }

    pub fn warning(msg: &str) {
        println!("{} {}", "⚠".yellow().bold(), msg);
    }

    pub fn header(msg: &str) {
        println!("\n{}", msg.bold().cyan());
        println!("{}", "─".repeat(msg.len()).cyan());
    }

    pub fn input(prompt: &str) -> Result<String, io::Error> {
        Input::new().with_prompt(prompt).interact_text()
    }

    pub fn input_optional(prompt: &str) -> Result<Option<String>, io::Error> {
        let input: String = Input::new()
            .with_prompt(prompt)
            .allow_empty(true)
            .interact_text()?;

        if input.is_empty() {
            Ok(None)
        } else {
            Ok(Some(input))
        }
    }

    pub fn password(prompt: &str) -> Result<String, io::Error> {
        Password::new().with_prompt(prompt).interact()
    }

    pub fn confirm(prompt: &str) -> Result<bool, io::Error> {
        Confirm::new().with_prompt(prompt).interact()
    }

    pub fn select(prompt: &str, items: &[&str]) -> Result<usize, io::Error> {
        Select::new().with_prompt(prompt).items(items).interact()
    }

    pub fn progress_bar(len: u64, msg: &str) -> ProgressBar {
        let pb = ProgressBar::new(len);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}")
                .unwrap()
                .progress_chars("##-")
        );
        pb.set_message(msg.to_string());
        pb
    }

    pub fn table_row(key: &str, value: &str) {
        println!("{:20} {}", key.cyan(), value);
    }

    pub fn print_banner() {
        println!(
            "{}",
            r#"
  _____ __  __  ____  _      _____          _____ ______ 
 / ____|  \/  |/ __ \| |    / ____|   /\   / ____|  ____|
| (___ | \  / | |  | | |   | |       /  \ | (___ | |__   
 \___ \| |\/| | |  | | |   | |      / /\ \ \___ \|  __|  
 ____) | |  | | |__| | |___| |____ / ____ \____) | |____ 
|_____/|_|  |_|\____/|______\_____/_/    \_\_____/______|
"#
            .bold()
            .yellow()
        );
        println!("{}", "Secure team secret management".italic());
        println!();
    }
}
