use std::process::Command;

pub fn get_default_git_remote(){
    let output = Command::new("git")
        .arg("remote")
        .arg("get-url")
        .arg("origin")
        .output()
        .expect("failed to execute process");
    let output_str = String::from_utf8_lossy(&output.stdout).to_string();
    webbrowser::open(&output_str).unwrap();
}

pub fn get_git_user_name() -> String {
    let output = Command::new("git")
        .arg("config")
        .arg("user.name")
        .output()
        .expect("failed to execute process");
    let output_str = String::from_utf8_lossy(&output.stdout).to_string();
    output_str
}

pub fn get_current_branch() -> String {
    let output = Command::new("git")
        .arg("branch")
        .arg("--show-current")
        .output()
        .expect("failed to execute process");
    let output_str = String::from_utf8_lossy(&output.stdout).to_string();
    output_str
}