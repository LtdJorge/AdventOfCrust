use std::{fs::File, io::Read, path::PathBuf};

#[derive(Clone)]
pub enum InputType {
    Test,
    Input(PathBuf),
}

pub fn get_input(input_type: InputType) -> anyhow::Result<String> {
    let path = match input_type {
        InputType::Test => PathBuf::from("./test.txt"),
        InputType::Input(path) => path,
    };
    let mut file = File::open(path)?;
    let mut file_slice = String::new();
    file.read_to_string(&mut file_slice)?;
    Ok(file_slice)
}
