use std::path::PathBuf;

use futures::{Stream, StreamExt};
use tokio::fs;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum DataSrc {
    Local {
        src: String,
        total: usize,
        entries: usize,
        highlights: usize,
    },
}

impl DataSrc {
    pub async fn open_local(path: PathBuf) -> Result<Self, std::io::Error> {
        let src = path.to_str().unwrap().to_string();

        let mut read_dir = fs::read_dir(path).await?;
        let mut total = 0;
        let mut entries = 0;
        let mut highlights = 0;
        while let Some(entry) = read_dir.next_entry().await? {
            let path = entry.path();
            let contents = fs::read(path).await?;

            let mut rdr = csv_async::AsyncReader::from_reader(contents.as_slice());
            let mut records = rdr.records();

            let name = entry.file_name();
            let file_name = name
                .to_str()
                .ok_or(std::io::Error::other("expected a file"))?;

            while let Some(r) = records.next().await {
                match r {
                    Ok(_) => {
                        if file_name.ends_with(".inputs.csv") {
                            total += 1;
                        } else if file_name.ends_with(".outputs.csv") {
                            entries += 1;
                        } else if file_name.ends_with(".highlights.csv") {
                            highlights += 1;
                        } else {
                            return Err(std::io::Error::other(format!(
                                "unexpected entry: {:?}",
                                entry.path()
                            )));
                        }
                    }
                    Err(e) => {
                        return Err(std::io::Error::other(format!("csv parsing error: {:?}", e)));
                    }
                }
            }
        }

        Ok(Self::Local {
            src,
            total,
            entries,
            highlights,
        })
    }

    // pub async fn entries_count(&self) -> usize {

    // }
}
