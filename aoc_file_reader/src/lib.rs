//! Helper library to read the input files for each day.

/// Path to the daily Advent of Code files.
const PATH: &str = "../res";
// const PATH: &str = "C:/Development/Rust/Advent_of_Code/AoC_2023/res";

/// This reads the file from the const PATH, currently '../res',
/// which is a folder under the year '2023/res'.
pub fn read_file(filename: &str) -> String {
    let mut file_path = PATH.to_owned() + "/" + filename;
    //check file exists
    let mut path = std::path::Path::new(&file_path);
    // dbg!(std::env::current_dir().unwrap());
    // dbg!(&path);
    if !path.exists() {
        // check one level lower
        if file_path.starts_with("..") {
            file_path = file_path[3..].to_owned();
            path = std::path::Path::new(&file_path);
            if !path.exists() {
                panic!("Could not find file: {file_path}");
            }
        }
    }
    // return input
    std::fs::read_to_string(path).expect(format!("Could not read file: {file_path}").as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_read() {
        let input = read_file("input_day_01.txt");
        let line_count = input.lines().count();
        assert_ne!(line_count, 0);
    }
}
