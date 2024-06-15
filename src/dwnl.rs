use std::fs::File;
use std::io::copy;
use std::error::Error;
use reqwest::blocking::Client;

pub fn file(url: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    let response = Client::new().get(url).send()?;
    let mut dest = File::create(output_path)?;
    copy(&mut response.bytes()?.as_ref(), &mut dest)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Read;

    #[test]
    fn test_download_file() {
        let url = "https://raw.githubusercontent.com/iruzo/msailor/main/Cargo.toml";
        let output_path = "test_output_file.txt";

        let result = file(url, output_path);
        assert!(result.is_ok());

        let mut file = File::open(output_path).expect("Failed to open the file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read the file");

        assert!(contents.contains("msailor"));

        fs::remove_file(output_path).expect("Failed to delete the test file");
    }
}
