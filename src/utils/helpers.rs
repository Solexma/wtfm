use crate::error::WtfmError;
use colored::*;
use regex::Regex;

pub struct DisplayHelper;

impl DisplayHelper {
    pub fn show_tips(tips: &str) {
        println!("\n{}", "TIPS:".bright_blue().bold());
        for line in tips.lines() {
            if line.starts_with('-') {
                println!("  {}", line);
            } else if line.is_empty() {
                println!();
            } else {
                println!("{}", line.bold());
            }
        }
        println!();
    }

    pub fn show_error(message: &str) {
        println!("{} {}", "ERROR:".bright_red().bold(), message);
    }

    pub fn show_success(message: &str) {
        println!("{} {}", "SUCCESS:".bright_green().bold(), message);
    }

    pub fn show_warning(message: &str) {
        println!("{} {}", "WARNING:".bright_yellow().bold(), message);
    }

    pub fn show_section_header(title: &str) {
        println!("\n{}", "=".repeat(50).bright_blue());
        println!("{}", title.bold());
        println!("{}\n", "=".repeat(50).bright_blue());
    }
}

pub struct ValidationHelper;

impl ValidationHelper {
    pub fn validate_email(email: &str) -> Result<(), WtfmError> {
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        if email_regex.is_match(email) {
            Ok(())
        } else {
            Err(WtfmError::InvalidEmail(email.to_string()))
        }
    }

    pub fn validate_project_name(name: &str) -> Result<(), String> {
        if name.trim().is_empty() {
            return Err("Project name cannot be empty".to_string());
        }
        Ok(())
    }

    pub fn validate_version(version: &str) -> Result<(), WtfmError> {
        let version_regex = Regex::new(r"^\d+\.\d+\.\d+$").unwrap();
        if version_regex.is_match(version) {
            Ok(())
        } else {
            Err(WtfmError::InvalidVersion(version.to_string()))
        }
    }
}
