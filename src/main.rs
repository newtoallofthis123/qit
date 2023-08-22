use clap::Parser;

#[derive(Parser, Debug)]
#[command(name="Qit", author="Ishan Joshi", version, about="Quickly Commit Stuff", long_about = None)]

//? The Args struct is used to parse the command line arguments
struct Args {
    #[arg(required=true, help="The commit message")]
    commit: String,

    #[arg(short, long, help="The Branch to commit to")]
    branch: Option<String>
}

fn main() {
    let args = Args::parse();
    bunt::println!("Quick Git doing it's {$underline}thing{/$} 😉...");
    bunt::println!("Parsing {$blue}Arguments{/$}...");
    let commit_msg = args.commit;
    //run the git commands
    let mut git_add = std::process::Command::new("git");
    git_add.arg("add").arg(".");
    git_add.output().expect("Failed to add files");
    let mut git_commit = std::process::Command::new("git");
    git_commit.arg("commit").arg("-m").arg(commit_msg.clone());
    git_commit.output().expect("Failed to commit");
    bunt::println!("Commited to Git with message: {$green}{}{/$}", commit_msg.clone());
    let wanna_commit = inquire::Text::new("Do you want to push to remote?");
    let wanna_commit = wanna_commit.with_help_message("Enter y or n");
    if wanna_commit.prompt().unwrap() == "y"{
        if args.branch.is_some(){
            // first check if the branch exists
            let branch = args.branch.unwrap();
            let mut git_branch = std::process::Command::new("git");
            git_branch.arg("branch").arg("-a");
            let output = git_branch.output().expect("Failed to get branches");
            let output = String::from_utf8(output.stdout).unwrap();
            let branches: Vec<&str> = output.split("\n").collect();
            let mut branch_exists = false;
            for b in branches{
                if b.trim() == branch{
                    branch_exists = true;
                    break;
                }
            }
            if !branch_exists{
                bunt::println!("{$red}Branch does not exist{/$}");
                std::process::exit(1);
            }
            let mut git_push = std::process::Command::new("git");
            git_push.arg("push").arg("origin").arg(branch);
            git_push.output().expect("Failed to push");
        }
        else{
            let mut git_push = std::process::Command::new("git");
            git_push.arg("push");
            git_push.output().expect("Failed to push");
        }
        bunt::println!("{$green}Pushed to remote{/$}");
    }
    else{
        bunt::println!("{$red}Not Pushed to remote{/$}");
        bunt::println!("{$yellow}However, I already committed it{/$}");
        let wanna_undo = inquire::Text::new("Do you want to undo the commit?");
        let wanna_undo = wanna_undo.with_help_message("Enter y or n");
        if wanna_undo.prompt().unwrap() == "y"{
            let mut git_undo = std::process::Command::new("git");
            git_undo.arg("reset").arg("HEAD~1").arg("--hard");
            git_undo.output().expect("Failed to undo");
            bunt::println!("{$green}Undid the commit{/$}");
        }
        else{
            bunt::println!("{$red}Not Undone{/$}");
        }
    }
    bunt::println!("{$green}Done{/$}");
    std::process::exit(0);
}
