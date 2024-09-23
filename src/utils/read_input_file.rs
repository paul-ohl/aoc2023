pub fn read_input_file(file_name: &str) -> String {
    let file_path = format!("./inputs/{}", file_name);
    std::fs::read_to_string(file_path).expect("Error reading file")
}
