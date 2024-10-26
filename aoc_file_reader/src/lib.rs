/*!
MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---

Helper library to read the input files for each day. It returns the file as String.

The file name is defined in the lib of the current day, the path is defined here.

*/

/// Path to the daily Advent of Code files.
const AOC_RESOURCE_PATH: &str = "../res";
// const PATH: &str = "C:/Development/Rust/Advent_of_Code/AoC_2023/res";

/// This reads the file from the const AOC_RESOURCE_PATH, currently '../res', which is
/// a folder under the year, e.g. '2023/res' and returns its content as String.
/// If the file does not exist, it tries to read it from the current directory.
pub fn read_file(filename: &str) -> String {
    let mut file_path = AOC_RESOURCE_PATH.to_owned() + "/" + filename;
    let path = std::path::Path::new(&file_path);
    if !path.exists() && !path.is_absolute() {
        // check current dir
        let file_name = path.file_name().unwrap();
        let curr_path = std::env::current_dir().unwrap().join(file_name);
        if curr_path.exists() {
            file_path = curr_path.to_str().unwrap().to_owned();
        }
    }

    read_file_path(&file_path)
}

/// This reads the file from the given path and returns its content as String.
pub fn read_file_path(file_path: &str) -> String {
    //check file exists
    let path = std::path::Path::new(&file_path);
    // dbg!(std::env::current_dir().unwrap());
    // dbg!(&path);
    // return file content as String
    std::fs::read_to_string(path).unwrap_or_else(|_| {
        if path.is_absolute() {
            panic!("Could not read file: {:?}", path)
        } else {
            // let full_path = std::env::current_dir().unwrap().join(path);
            panic!(
                "Current dir: {}\nCould not read file: {}",
                std::env::current_dir()
                    .unwrap()
                    .as_os_str()
                    .to_string_lossy(),
                path.to_string_lossy()
            )
        };
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE_PATH: &str = "../2023/res/input_day_00.txt";

    #[test]
    fn test_file_read() {
        let input = read_file("cargo.toml");
        let line_count = input.lines().count();
        assert_ne!(line_count, 0);
    }

    #[test]
    fn test_file_read_path() {
        let input = read_file_path(FILE_PATH);
        let line_count = input.lines().count();
        assert_ne!(line_count, 0);
    }
}
