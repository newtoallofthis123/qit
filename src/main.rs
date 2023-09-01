use clap::Parser;

#[derive(Parser, Debug)]
#[command(name="Qit", author="Ishan Joshi", version, about="Git for beginners", long_about = None)]

//? The Args struct is used to parse the command line arguments
struct Args {
    #[arg(required=false, help="Any Basic Git Command")]
    cmd: Option<String>,

    #[arg(required=false, help="Any Sub command", default_value="?")]
    sub: Option<String>
}

mod handler;
mod actions;
mod cli;
mod license;

fn main() {
    bunt::println!("{$blue}   ____    _ __ {/$}");
    bunt::println!("{$yellow}  / __ \\  (_) /_{/$}");
    bunt::println!("{$red} / / / / / / __/{/$}");
    bunt::println!("{$yellow}/ /_/ / / / /_  {/$}");
    bunt::println!("{$blue}\\___\\_\\/_/\\__/ {/$}{$green}v.1{/$} {$underline}{$bold}Simple Git{/$}{/$} for Beginners\n");
    let args = Args::parse();
    let cmd:String;
    if args.cmd.is_none() {
        cmd = inquire::Text::new("Enter a command or commit message").prompt().unwrap();
    }
    else {
        cmd = args.cmd.unwrap();
    }
    match cmd.as_str() {
        "open" => {
            bunt::println!("Opening the git repository in your {$blue}default browser{/$}...");
            actions::get_default_git_remote();    
        }
        "init" => {
            
        }
        _ => {
            bunt::println!("No Command Detected, proceeding to {$yellow}commit{/$} message")
        }
    }


}
