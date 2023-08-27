use std::process::Command;

pub fn get_all_branchs() -> Vec<String> { 
    let output = Command::new("git")
        .arg("branch")
        .output()
        .expect("failed to execute process");
    let output_str = String::from_utf8_lossy(&output.stdout).to_string();
    let output_vec: Vec<String> = output_str.split("\n").map(|s| s.to_string()).collect();
    let result: Vec<String> = output_vec.iter().map(|s| {
        s.replace("* ", "").replace(" ", "")
    }).collect();
    result[..result.len()-1].to_vec()
}

pub fn get_current_branch() ->String{
    get_all_branchs()[0].to_string()
}

pub fn get_all_files() -> Vec<String> {
    let output = Command::new("git")
        .arg("ls-files")
        .output()
        .expect("failed to execute process");
    let output_str = String::from_utf8_lossy(&output.stdout).to_string();
    let output_vec: Vec<String> = output_str.split("\n").map(|s| s.to_string()).collect();
    output_vec[..output_vec.len()-1].to_vec()
}

pub fn get_modifiled_files() -> Vec<String> {
    let output = Command::new("git")
        .arg("ls-files")
        .arg("--modified")
        .output()
        .expect("failed to execute process");
    let output_str = String::from_utf8_lossy(&output.stdout).to_string();
    let output_vec: Vec<String> = output_str.split("\n").map(|s| s.to_string()).collect();
    output_vec[..output_vec.len()-1].to_vec()
}


pub fn git_commit(msg: &str)-> String {
    let output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(msg)
        .output()
        .expect("failed to execute process");
    let output_str = String::from_utf8_lossy(&output.stdout).to_string();
    output_str
}

pub fn git_add(file :&str){
    let mut output = Command::new("git");
    output.arg("add");
    if file == "."{
            output.arg(".");
    }
    else {
        output.arg(file);
    }
    let output = output.output()
        .expect("failed to execute process");
    let output_str = String::from_utf8_lossy(&output.stdout).to_string();
    bunt::println!("{}", output_str);
}

pub fn git_push(branch: &str) -> String {
    let mut output = Command::new("git");
    output.arg("push");
    if branch != ""{
        output.arg("-u");
        output.arg("origin");
        output.arg(branch);
    }
    let output = output.output()
        .expect("failed to execute process");
    let output_str = String::from_utf8_lossy(&output.stdout).to_string();
    output_str
}

pub fn show_status(){
    let output = Command::new("git")
        .arg("status")
        .output()
        .expect("failed to execute process");
    let output_str = String::from_utf8_lossy(&output.stdout).to_string();
    bunt::println!("{}", output_str);
}

pub fn is_tree_clean() -> bool {
    //TODO: Check if tree is clean
    true
}

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