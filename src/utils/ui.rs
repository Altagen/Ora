use console::style;
use indicatif::{ProgressBar, ProgressStyle};

// UI helper functions - planned for future CLI improvements
#[allow(dead_code)]
pub fn success(msg: &str) {
    println!("{} {}", style("✅").green().bold(), msg);
}

#[allow(dead_code)]
pub fn error(msg: &str) {
    eprintln!("{} {}", style("❌").red().bold(), msg);
}

#[allow(dead_code)]
pub fn warning(msg: &str) {
    println!("{} {}", style("⚠").yellow().bold(), msg);
}

#[allow(dead_code)]
pub fn info(msg: &str) {
    println!("{} {}", style("ℹ").blue().bold(), msg);
}

#[allow(dead_code)]
pub fn create_progress_bar(len: u64) -> ProgressBar {
    let pb = ProgressBar::new(len);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            // NOTE: This expect() is acceptable because the template string is a compile-time constant.
            // If the template is invalid, it's a programmer error and should fail fast at startup.
            .expect("BUG: Invalid hardcoded progress bar template")
            .progress_chars("#>-"),
    );
    pb
}

#[allow(dead_code)]
pub fn create_spinner(msg: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            // NOTE: This expect() is acceptable because the template string is a compile-time constant.
            // If the template is invalid, it's a programmer error and should fail fast at startup.
            .expect("BUG: Invalid hardcoded spinner template"),
    );
    pb.set_message(msg.to_string());
    pb
}
