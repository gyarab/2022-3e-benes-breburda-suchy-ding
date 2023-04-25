use async_std::{path::PathBuf, fs::File};
use async_std::fs;
use ring::rand::{SystemRandom, SecureRandom};
use base64::{Engine as _, engine::general_purpose as base64_coder};


#[derive(Clone)]
pub struct FileManager {
    pub directory: PathBuf
}

impl FileManager {
    pub async fn new(dir: &str) -> Self {
        let directory = PathBuf::from(dir);
        if !directory.is_dir().await {
            panic!("upload directory is not not a directory");
        }
        Self {
            directory
        }
    }

    pub async fn get_body(&self, file: &str) -> anyhow::Result<tide::Body> {
        let file = self.directory.join(file);
        Ok(tide::Body::from_file(file).await?)
    }

    pub async fn create_file(&self, stream: &mut tide::Body) -> anyhow::Result<String> {
        let filename = {
            let random = SystemRandom::new();
            let mut rndm = [0u8; 8];
            random.fill(&mut rndm)?;
            base64_coder::URL_SAFE_NO_PAD.encode(rndm)
        };

        let mut file = File::create(self.directory.join(&filename)).await?;
        async_std::io::copy(stream, &mut file).await?;

        Ok(filename)
    }

    pub fn get_path(&self, file: &str) -> PathBuf {
        self.directory.join(file)
    }

    pub async fn delete_file(&self, file: &str) -> anyhow::Result<()> {
        let path = self.directory.join(file);
        fs::remove_file(path).await?;
        Ok(())
    }
}
