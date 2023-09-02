pub fn show_options(options: Vec<&str>) -> &str {
    let result = inquire::Select::new("Select an option", options).with_page_size(10).prompt().unwrap();
    result
}

pub fn get_conformation(msg: &str) -> bool {
    let result = inquire::Confirm::new(msg).with_default(true).prompt().unwrap();
    result
}

pub fn _get_input(msg: &str) -> String {
    let result = inquire::Text::new(msg).prompt().unwrap();
    result
}