use std::process::Command;
use std::fs::File;

pub fn handle_init() {
    bunt::println!("The {$blue}Magical{/$} Init Command");
    //first we git init
    Command::new("git")
        .arg("init")
        .output()
        .expect("failed to execute process");
    bunt::println!("{$green}Git Init{/$} Successful");
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
        bunt::println!("{$green}Git Remote{/$} Added Successfully");
    }
    else {
        bunt::println!("{$red}Git Remote{/$} Not Added");
    }
    //then we add a readme
    File::create("README.md").expect("Unable to create the README.md");
    bunt::println!("{$green}README.md{/$} Created Successfully");
    // readme content
    let readme_title = inquire::Text::new("Enter the title of the README").prompt().unwrap();
    let readme_description = inquire::Text::new("Enter the description of the README").prompt().unwrap();
    let readme_content = format!("# {}\n\n{}", readme_title, readme_description);
    
    std::fs::write(std::path::Path::new("README.md"), readme_content).expect("Unable to write to the README.md");
    bunt::println!("{$green}README.md{/$} Content Added Successfully");

    //then we add a gitignore
    File::create(".gitignore").expect("Unable to create the .gitignore");
    bunt::println!("{$green}.gitignore{/$} Created Successfully");
    bunt::println!("{$red}TODO:{/$} Add language specific gitignore content");
    
    //then we add a license
    
}
