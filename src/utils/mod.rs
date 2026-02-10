use std::collections::HashSet;

use crate::models::message_node::MessageNode;

pub mod connector;

pub fn deduplicate_message_nodes(message_nodes: Vec<MessageNode>) -> Vec<MessageNode> {
    let mut unique_nodes = HashSet::new();
    let mut deduplicated = Vec::new();

    let message_nodes = message_nodes.iter().filter(|node| node.content.is_some());
    for node in message_nodes {
        if let Some(content) = node.content.clone() {
            let content = content.trim().to_string();
            if unique_nodes.insert(content) {
                deduplicated.push(node.clone());
            }
        }
    }
    deduplicated
}

/// Computes cosine similarity between two vectors.
/// Returns a value between -1.0 and 1.0, where 1.0 means identical direction.
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f64 {
    assert_eq!(a.len(), b.len(), "vectors must have same length");

    let dot_product: f64 = a.iter().zip(b.iter()).map(|(x, y)| (*x as f64) * (*y as f64)).sum();
    let norm_a: f64 = a.iter().map(|x| (*x as f64).powi(2)).sum::<f64>().sqrt();
    let norm_b: f64 = b.iter().map(|x| (*x as f64).powi(2)).sum::<f64>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    dot_product / (norm_a * norm_b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cosine_similarity_identical_vectors() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![1.0, 2.0, 3.0];
        let sim = cosine_similarity(&a, &b);
        assert!((sim - 1.0).abs() < 1e-10, "identical vectors should have similarity 1.0");
    }

    #[test]
    fn cosine_similarity_opposite_vectors() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![-1.0, -2.0, -3.0];
        let sim = cosine_similarity(&a, &b);
        assert!((sim - (-1.0)).abs() < 1e-10, "opposite vectors should have similarity -1.0");
    }

    #[test]
    fn cosine_similarity_orthogonal_vectors() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![0.0, 1.0, 0.0];
        let sim = cosine_similarity(&a, &b);
        assert!(sim.abs() < 1e-10, "orthogonal vectors should have similarity 0.0");
    }

    #[test]
    fn cosine_similarity_scaled_vectors() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![2.0, 4.0, 6.0];
        let sim = cosine_similarity(&a, &b);
        assert!((sim - 1.0).abs() < 1e-10, "scaled vectors should have similarity 1.0");
    }

    #[test]
    fn cosine_similarity_zero_vector() {
        let a = vec![0.0, 0.0, 0.0];
        let b = vec![1.0, 2.0, 3.0];
        let sim = cosine_similarity(&a, &b);
        assert_eq!(sim, 0.0, "zero vector should have similarity 0.0");
    }
}
