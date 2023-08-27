pub fn show_options(options: Vec<String>) -> String {
    let result = inquire::Select::new("Select an option", options).with_page_size(10).prompt().unwrap();
    result
}

pub fn get_conformation(msg: &str) -> bool {
    let result = inquire::Confirm::new(msg).prompt().unwrap();
    result
}