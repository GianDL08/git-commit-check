use clap::Parser;
use regex::Regex;
use std::fs;
use std::process::{Command, ExitCode};

#[derive(Parser)]
#[command(name = "git-commit-check", version, about = "Validate git commit messages")]
struct Args {
    /// Path to the commit message file
    commit_msg_file: Option<String>,
}

fn main() -> ExitCode {
    let args = Args::parse();

    let message = match args.commit_msg_file {
        Some(path) => match fs::read_to_string(&path) {
            Ok(contents) => contents,
            Err(err) => {
                eprintln!("Failed to read commit message file '{path}': {err}");
                return ExitCode::from(1);
            }
        },
        None => match read_latest_commit_message() {
            Ok(contents) => contents,
            Err(err) => {
                eprintln!("Failed to read latest commit message: {err}");
                return ExitCode::from(1);
            }
        },
    };

    let subject = message.lines().next().unwrap_or("").trim_end();
    let errors = validate_subject(subject);

    if errors.is_empty() {
        println!("Commit message is valid.");
        ExitCode::from(0)
    } else {
        eprintln!("Commit message is invalid:");
        for error in errors {
            eprintln!("- {error}");
        }
        ExitCode::from(1)
    }
}

fn read_latest_commit_message() -> Result<String, String> {
    let output = Command::new("git")
        .args(["log", "-1", "--pretty=%B"])
        .output()
        .map_err(|err| err.to_string())?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(stderr.trim().to_string());
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn validate_subject(subject: &str) -> Vec<String> {
    let mut errors = Vec::new();

    if subject.is_empty() {
        errors.push("Subject line is empty".to_string());
        return errors;
    }

    let subject_len = subject.chars().count();
    if subject_len > 72 {
        errors.push(format!(
            "Subject line is {subject_len} characters; must be 72 or fewer"
        ));
    }

    let re = Regex::new(r"^(feat|fix|docs|style|refactor|test|chore)(\([^)]+\))?: (.+)$")
        .expect("regex must compile");

    if let Some(caps) = re.captures(subject) {
        let description = caps.get(3).map(|m| m.as_str()).unwrap_or("");

        if description.ends_with('.') {
            errors.push("Description must not end with a period".to_string());
        }

        if description != description.to_lowercase() {
            errors.push("Description must be lowercase".to_string());
        }
    } else {
        errors.push(
            "Subject must match <type>(<optional-scope>): <description> with a valid type".to_string(),
        );
    }

    errors
}
