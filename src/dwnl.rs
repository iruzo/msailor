use std::error::Error;
use tokio::fs::File as TokioFile;
use tokio::io::AsyncWriteExt;
use reqwest::Client;

pub async fn file(url: String, output_path: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    let response = Client::new().get(&url).send().await?;
    let bytes = response.bytes().await?;
    let mut dest = TokioFile::create(output_path).await?;
    dest.write_all(&bytes).await?;
    Ok(())
}

pub async fn download_files(urls: Vec<(String, String)>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut tasks = vec![];
    for (url, path) in urls {
        let task = tokio::spawn(async move {
            file(url, path).await
        });
        tasks.push(task);
    }
    for task in tasks {
        task.await??;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Read;

    #[tokio::test]
    async fn test_download_files() {
        let urls = vec![
            ("https://raw.githubusercontent.com/iruzo/msailor/main/Cargo.toml".to_string(), "test_output_file_1.txt".to_string()),
            ("https://raw.githubusercontent.com/iruzo/msailor/main/README.md".to_string(), "test_output_file_2.txt".to_string()),
        ];

        let result = download_files(urls).await;
        assert!(result.is_ok());

        let output_paths = vec!["test_output_file_1.txt", "test_output_file_2.txt"];
        for output_path in &output_paths {
            let mut file = fs::File::open(output_path).expect("Failed to open the file");
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Failed to read the file");

            assert!(contents.contains("msailor"));

            fs::remove_file(output_path).expect("Failed to delete the test file");
        }
    }
}

