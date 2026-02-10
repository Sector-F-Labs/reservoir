use std::path::PathBuf;

use anyhow::Error;
use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use tracing::info;

#[derive(Clone, Debug)]
pub struct EmbeddingInfo {
    pub model: String,
    pub length: i32,
}

impl EmbeddingInfo {
    pub fn with_fastembed(model: &str) -> Self {
        EmbeddingInfo {
            model: model.to_string(),
            length: 1024,
        }
    }

    pub fn get_node_name(&self) -> String {
        "Embedding1024".to_string()
    }

    pub fn get_index_name(&self) -> String {
        "embedding1024".to_string()
    }

    pub fn get_model_name(&self) -> String {
        self.model.clone()
    }
}

impl Default for EmbeddingInfo {
    fn default() -> Self {
        Self::with_fastembed("bge-large-en-v15")
    }
}

pub fn get_cache_path() -> PathBuf {
    let tmp_dir = dirs_next::data_dir().unwrap();
    tmp_dir.join("reservoir").join("models")
}

pub async fn get_embeddings_for_txt(
    text: &str,
    embedding_info: EmbeddingInfo,
) -> Result<Vec<f32>, Error> {
    info!("Using FastEmbed for embedding");
    info!("Embedding model: {}", embedding_info.model);
    info!("Embedding length: {}", embedding_info.length);

    let init_options = InitOptions::new(EmbeddingModel::BGELargeENV15)
        .with_show_download_progress(true)
        .with_cache_dir(get_cache_path());

    let model = TextEmbedding::try_new(init_options)?;
    let texts = vec![text];
    let embeddings = model.embed(texts, None)?;

    if let Some(embedding) = embeddings.first() {
        Ok(embedding.clone())
    } else {
        Err(Error::msg("No embeddings found"))
    }
}
