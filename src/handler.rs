use std::process::Command;
use std::fs::File;

pub fn check_existing_git_repo() -> bool {
    if std::path::Path::new(".git").exists() {
        return true;
    }
    false
}

pub fn check_existing_files() -> bool{
    if std::path::Path::new("README.md").exists() || std::path::Path::new("LICENSE").exists() || std::path::Path::new(".gitignore").exists() {
        return true;
    }
    false
}

pub fn handle_init() {
    bunt::println!("The {$blue}Magical{/$} Init Command");
    if check_existing_git_repo(){
        bunt::println!("{$red}Git Repository{/$} Already Exists. Continuing will {$red}purge{/$} the existing repository");
        let res = crate::cli::get_conformation("Do you want to continue?");
        if !res {
            std::process::exit(0);
        }
        else{
            bunt::println!("{$yellow}Purging{/$} the existing repository");
            handle_purge();
            println!("---------------------");
        }
    }
    if check_existing_files(){
        bunt::println!("{$red}Existing Files{/$} Already Exist. Continuing will {$red}overwrite{/$} the existing files");
        let res = crate::cli::get_conformation("Do you want to continue?");
        if !res {
            std::process::exit(0);
        }
        else{
            bunt::println!("Continuing...");
        }
    }
    //first we git init
    Command::new("git")
        .arg("init")
        .output()
        .expect("failed to execute process");
    bunt::println!("{$yellow}Git Init{/$} Successful");
    //then we add the remote
    let remote_res = crate::cli::get_conformation("Do you want to add a remote?");
    if remote_res {
        let remote = inquire::Text::new("Enter the remote url").prompt().unwrap();
        Command::new("git")
            .arg("remote")
            .arg("add")
            .arg("origin")
            .arg(&remote)
            .output()
            .expect("failed to execute process");
        bunt::println!("{$yellow}Git Remote{/$} Added Successfully");
    }
    else {
        bunt::println!("{$red}Git Remote{/$} Not Added");
    }
    //then we add a readme
    File::create("README.md").expect("Unable to create the README.md");
    // readme content
    let readme_title = inquire::Text::new("The Name of your project").prompt().unwrap();
    let readme_description = inquire::Text::new("A short description of your project").prompt().unwrap();
    let readme_content = format!("# {}\n\n{}", readme_title, readme_description);
    
    std::fs::write(std::path::Path::new("README.md"), readme_content).expect("Unable to write to the README.md");
    bunt::println!("{$yellow}README.md{/$} Content Added Successfully");

    //then we add a gitignore
    std::fs::write(std::path::Path::new(".gitignore"), "# Please fill out this .gitignore on your own. More support coming soon :)").expect("Unable to write to the .gitignore");
    bunt::println!("{$yellow}.gitignore{/$} Created Successfully");
    
    //then we add a license
    //get the license text first from the github api
    let license = crate::cli::show_options(vec!["mit", "apache-2.0", "gpl-3.0", "unlicense"]);
    let name = inquire::Text::new("Enter your name").with_default(&crate::actions::get_git_user_name()).prompt().unwrap();
    let year = inquire::Text::new("Enter the year").with_default(time::OffsetDateTime::now_utc().year().to_string().as_str()).prompt().unwrap();
    let license_text = crate::license::get_license_text(&license, &year, &name);
    //then we write the license text to the LICENSE file
    File::create("LICENSE").expect("Unable to create the LICENSE");
    std::fs::write(std::path::Path::new("LICENSE"), license_text).expect("Unable to write to the LICENSE");
    bunt::println!("{$green}✔{/$}{$yellow}License{/$} Created Successfully");

    bunt::println!("{$green}Git Init{/$} Completed Successfully");
}

pub fn handle_purge(){
    bunt::println!("The {$blue}Magical{/$} Purge Command");
    
    if !std::path::Path::new(".git").exists() {
        bunt::println!("{$red}Purge{/$} Aborted. No {$yellow}.git{/$} directory found. You can't free a prisoner who doesn't exist");
        return;
    }

    let res = crate::cli::get_conformation("Are you sure you want to purge this repository?");
    if res {
        bunt::println!("Purging the repository...");
        std::fs::remove_dir_all(".git").expect("Unable to purge the repository");
        bunt::println!("Purged the {$green}hidden .git{/$}");
        bunt::println!("{$red}Purge{/$} Completed Successfully. You are now free to initialize a new git repository");
    }
    else {
        bunt::println!("{$red}Purge{/$} Aborted");
    }
}

pub fn handle_normal(commit_msg: &str){
    bunt::println!("Proceeding to commit on the {$yellow}{}{/$} branch", crate::actions::get_current_branch());
    Command::new("git")
        .arg("add")
        .arg(".")
        .output()
        .expect("failed to execute process");
    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(&commit_msg)
        .output()
        .expect("failed to execute process");
    bunt::println!("{$green}Commit{/$} Successful");
    let res = crate::cli::get_conformation("Do you want to push the changes?");
    if !res {
        bunt::println!("{$red}Push{/$} Aborted");
        return;
    }
    Command::new("git")
        .arg("push")
        .arg("origin")
        .output()
        .expect("failed to execute process");
    bunt::println!("{$green}Commit{/$} Successful");
}