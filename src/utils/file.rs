use crate::SgResult;
use log::debug;
use std::path::PathBuf;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct FileUtils;

impl FileUtils {
    pub async fn read_file(path: &PathBuf) -> SgResult<String> {
        let mut file = OpenOptions::new()
            .read(true)
            .create(true)
            .append(true)
            .write(true)
            .open(path)
            .await?;

        let mut string = String::new();

        file.read_to_string(&mut string).await?;

        Ok(string)
    }

    pub async fn prepend_to_file(path: &PathBuf, content: String) -> SgResult<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(path)
            .await?;

        let contents = FileUtils::read_file(&path).await?;

        debug!("contents {}", contents);

        file.write_all(content.as_bytes()).await?;
        file.write_all(contents.as_bytes()).await?;

        Ok(())
    }

    pub async fn write(path: &PathBuf, content: String) -> SgResult<()> {
        let mut file = OpenOptions::new()
            .read(true)
            .create(true)
            .append(true)
            .write(true)
            .open(path)
            .await?;

        file.write_all(content.as_bytes()).await?;

        Ok(())
    }
}
