use clap::Parser;

#[derive(Parser, Debug)]
#[command(name="Qit", author="Ishan Joshi", version, about="Git for beginners", long_about = None)]

//? The Args struct is used to parse the command line arguments
struct Args {
    #[arg(required=false, help="Any Basic Git Command")]
    cmd: Option<String>,

    #[arg(required=false, help="Any Sub command", default_value="?")]
    sub: Option<String>,

    #[arg(required=false, short, long, help="Opens the git repository in your default browser")]
    open: bool,
}

mod handler;
mod actions;
mod cli;

fn main() {
    bunt::println!("{$blue}   ____    _ __ {/$}");
    bunt::println!("{$yellow}  / __ \\  (_) /_{/$}");
    bunt::println!("{$red} / / / / / / __/{/$}");
    bunt::println!("{$yellow}/ /_/ / / / /_  {/$}");
    bunt::println!("{$blue}\\___\\_\\/_/\\__/ {/$}{$green}v.1{/$} {$underline}{$bold}Simple Git{/$}{/$} for Beginners\n");
    let args = Args::parse();
    if args.open {
        bunt::println!("Opening the git repository in your {$blue}default browser{/$}...");
        actions::get_default_git_remote();
        return;
    }
    let cmd:String;
    if args.cmd.is_none() {
        cmd = cli::show_options(vec!["add".to_string(), "commit".to_string(), "push".to_string(), "checkout".to_string(), "help".to_string()]);
    }
    else {
        cmd = args.cmd.unwrap();
    }

    match cmd.as_str() {
        "add" => {
            let file = args.sub.unwrap();
            handler::add(&file);
        }
        "commit" => {
            let msg:String;
            if args.sub.is_none() {
                msg = inquire::Text::new("Enter Commit Message").prompt().unwrap();
            }
            else {
                msg = args.sub.unwrap();
            }
            handler::commit(&msg);
        }
        "push" => {
            handler::push();
        }
        "checkout" => {
            handler::checkout(&args.sub.unwrap());
        }
        "help" => {
            bunt::println!("Git {$yellow}Help{/$}");
            bunt::println!("{$underline}Qit{/$} is a simple git wrapper for beginners, use --open to open the git repository in your default browser");
            bunt::println!("{$green}add{/$} - Adds a file to the staging area");
            bunt::println!("{$yellow}commit{/$} - Commits the staged files");
            bunt::println!("{$blue}push{/$} - Pushes the commited files to the remote repository");
            bunt::println!("{$red}checkout{/$} - Switches to a different branch");
        }
        _ => {
            bunt::println!("Invalid Command");
        }
    }
}
