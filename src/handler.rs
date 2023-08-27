use std::process::Command;

pub fn add(file: &str){
    bunt::println!("Git {$yellow}Adds{/$} a file to the staging area");
    let files = super::actions::get_all_files();
    let mut options: Vec<String> = files.iter().map(|s| s.to_string()).collect();
    options.push(".".to_string());
    options.reverse();
    let mut result = file.to_string();
    if result == "?"{
        result = super::cli::show_options(options);
    }
    else if result == "."{
        bunt::println!("{$yellow}Adding all files to staging area{/$}")
    }
    else{
        let mut found = false;
        for f in files{
            if f == result{
                found = true;
                break;
            }
        }
        if !found{
            bunt::println!("{$red}File does not exist{/$}");
            return;
        }
        else{
            bunt::println!("{$yellow}Adding file to staging area{/$}")
        }
    }
    super::actions::git_add(&result);
    bunt::println!("Added {} to staging area", result);   
}

pub fn commit(msg: &str){
    bunt::println!("Git {$yellow}Commits{/$} the staged files");
    let files = super::actions::get_modifiled_files();
    if files.len() == 0{
        bunt::println!("{$red}No files to commit{/$}");
        return;
    }
    bunt::println!("{$yellow}Files to commit{/$}");
    for f in files{
        bunt::println!("- {}", f);
    }
    let branch = super::actions::get_current_branch();
    bunt::println!("You are currently on {}", branch);
    let change_branch = super::cli::get_conformation("Do you want to change the branch?");
    if change_branch{
        checkout("?");
    }
    else{
        bunt::println!("{$yellow}Commiting on current branch{/$}");
    }
    let result = super::cli::get_conformation("Do you want to commit all the staged files?");
    if !result{
        bunt::println!("No Sweat...{$red}Aborting Commit{/$}");
        return;
    }
    super::actions::git_commit(msg);
    bunt::println!("Committed on current branch {} with message {}", super::actions::get_current_branch(), msg);
}

pub fn push(){
    bunt::println!("Git {$yellow}Pushes{/$} the commited files to the remote repository");
    let files = super::actions::get_modifiled_files();
    if files.len() == 0{
        bunt::println!("{$red}No files to push{/$}");
        return;
    }
    bunt::println!("{$yellow}Files to push{/$}");
    let branch = super::actions::get_current_branch();
    bunt::println!("You are currently on {}", branch);
    let change_branch = super::cli::get_conformation("Do you want to change the branch?");
    if change_branch{
        checkout("?");
    }
    let result = super::cli::get_conformation("Do you want to push to the remote repository?");
    if !result{
        bunt::println!("No Sweat...{$red}Aborting Push{/$}");
        return;
    }
    let output = super::actions::git_push(&branch);
    bunt::println!("{}", output);
}

pub fn checkout(branch: &str){
    bunt::println!("Git {$yellow}Checks Out{/$} to a different branch");
    let branches = super::actions::get_all_branchs();
    let mut options: Vec<String> = branches.iter().map(|s| s.to_string()).collect();
    options.reverse();
    let mut result = branch.to_string();
    if result == "?"{
        result = super::cli::show_options(options);
    }
    else{
        let mut found = false;
        for b in branches{
            if b == result{
                found = true;
                break;
            }
        }
        if !found{
            let create_branch = super::cli::get_conformation("Branch does not exist. Do you want to create it?");
            if create_branch{
                let output = Command::new("git")
                    .arg("checkout")
                    .arg("-b")
                    .arg(result)
                    .output()
                    .expect("failed to execute process");
                let output_str = String::from_utf8_lossy(&output.stdout).to_string();
                bunt::println!("{}", output_str);
                return;
            }
            else{
                bunt::println!("No Sweat...{$red}Aborting Checkout{/$}");
                return;
            }
        }
    }
    Command::new("git")
        .arg("checkout")
        .arg(result.clone())
        .output()
        .expect("failed to execute process");
    bunt::println!("Checked out to {}", result);
}