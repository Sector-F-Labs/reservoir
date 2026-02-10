pub mod neo4j_message;

use anyhow::Error;
use async_trait::async_trait;

use crate::models::message_node::MessageNode;

/// Trait abstracting message storage operations for testability.
/// Implementations can be backed by Neo4j, in-memory storage, or mocks.
#[async_trait]
pub trait MessageStore: Send + Sync {
    /// Gets the most recent message before the given timestamp in the same partition/instance.
    async fn get_previous_message(
        &self,
        partition: &str,
        instance: &str,
        before_timestamp: i64,
    ) -> Result<Option<MessageNode>, Error>;

    /// Creates a SYNAPSE relationship between two messages.
    /// Returns the similarity score if synapse was created.
    async fn create_synapse(
        &self,
        from_timestamp: i64,
        to_timestamp: i64,
        partition: &str,
        instance: &str,
        score: f64,
    ) -> Result<(), Error>;

    /// Gets the thread context by following SYNAPSE relationships backward from the latest message.
    async fn get_thread_from_latest(
        &self,
        partition: &str,
        instance: &str,
        max_count: usize,
    ) -> Result<Vec<MessageNode>, Error>;

    /// Saves a message node to storage.
    async fn save_message(
        &self,
        message: &MessageNode,
        embedding_node_name: &str,
        embedding_model_name: &str,
    ) -> Result<(), Error>;
}

#[cfg(test)]
pub mod mock {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Mutex;

    /// In-memory mock implementation of MessageStore for testing.
    pub struct MockMessageStore {
        messages: Mutex<Vec<MessageNode>>,
        synapses: Mutex<HashMap<(i64, i64), f64>>, // (from_ts, to_ts) -> score
    }

    impl MockMessageStore {
        pub fn new() -> Self {
            Self {
                messages: Mutex::new(Vec::new()),
                synapses: Mutex::new(HashMap::new()),
            }
        }

        pub fn with_messages(messages: Vec<MessageNode>) -> Self {
            Self {
                messages: Mutex::new(messages),
                synapses: Mutex::new(HashMap::new()),
            }
        }

        pub fn add_message(&self, message: MessageNode) {
            self.messages.lock().unwrap().push(message);
        }

        pub fn get_synapses(&self) -> Vec<((i64, i64), f64)> {
            self.synapses
                .lock()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, *v))
                .collect()
        }

        pub fn has_synapse(&self, from_ts: i64, to_ts: i64) -> bool {
            self.synapses.lock().unwrap().contains_key(&(from_ts, to_ts))
        }
    }

    impl Default for MockMessageStore {
        fn default() -> Self {
            Self::new()
        }
    }

    #[async_trait]
    impl MessageStore for MockMessageStore {
        async fn get_previous_message(
            &self,
            partition: &str,
            instance: &str,
            before_timestamp: i64,
        ) -> Result<Option<MessageNode>, Error> {
            let messages = self.messages.lock().unwrap();
            let prev = messages
                .iter()
                .filter(|m| {
                    m.partition == partition
                        && m.instance == instance
                        && m.timestamp < before_timestamp
                })
                .max_by_key(|m| m.timestamp)
                .cloned();
            Ok(prev)
        }

        async fn create_synapse(
            &self,
            from_timestamp: i64,
            to_timestamp: i64,
            _partition: &str,
            _instance: &str,
            score: f64,
        ) -> Result<(), Error> {
            self.synapses
                .lock()
                .unwrap()
                .insert((from_timestamp, to_timestamp), score);
            Ok(())
        }

        async fn get_thread_from_latest(
            &self,
            partition: &str,
            instance: &str,
            max_count: usize,
        ) -> Result<Vec<MessageNode>, Error> {
            let messages = self.messages.lock().unwrap();
            let synapses = self.synapses.lock().unwrap();

            // Find the latest message (no outgoing synapse)
            let partition_messages: Vec<_> = messages
                .iter()
                .filter(|m| m.partition == partition && m.instance == instance)
                .collect();

            let latest = partition_messages
                .iter()
                .filter(|m| {
                    // No outgoing synapse = latest in chain
                    !synapses.keys().any(|(from, _)| *from == m.timestamp)
                })
                .max_by_key(|m| m.timestamp);

            let Some(latest) = latest else {
                return Ok(Vec::new());
            };

            // Walk backward through synapses
            let mut thread = vec![(*latest).clone()];
            let mut current_ts = latest.timestamp;

            while thread.len() < max_count {
                // Find synapse pointing TO current
                let incoming = synapses
                    .iter()
                    .find(|((_, to), _)| *to == current_ts);

                if let Some(((from_ts, _), _)) = incoming {
                    if let Some(msg) = partition_messages.iter().find(|m| m.timestamp == *from_ts) {
                        thread.push((*msg).clone());
                        current_ts = *from_ts;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            // Return in chronological order (oldest first)
            thread.reverse();
            Ok(thread)
        }

        async fn save_message(
            &self,
            message: &MessageNode,
            _embedding_node_name: &str,
            _embedding_model_name: &str,
        ) -> Result<(), Error> {
            self.messages.lock().unwrap().push(message.clone());
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn make_message(partition: &str, instance: &str, ts: i64, content: &str) -> MessageNode {
            MessageNode {
                id: None,
                trace_id: format!("trace-{}", ts),
                partition: partition.to_string(),
                instance: instance.to_string(),
                content: Some(content.to_string()),
                role: "user".to_string(),
                embedding: vec![0.0; 1024],
                url: None,
                timestamp: ts,
            }
        }

        #[tokio::test]
        async fn get_previous_message_returns_most_recent_before_timestamp() {
            let store = MockMessageStore::new();
            store.add_message(make_message("p1", "i1", 100, "first"));
            store.add_message(make_message("p1", "i1", 200, "second"));
            store.add_message(make_message("p1", "i1", 300, "third"));

            let prev = store.get_previous_message("p1", "i1", 300).await.unwrap();
            assert!(prev.is_some());
            assert_eq!(prev.unwrap().timestamp, 200);
        }

        #[tokio::test]
        async fn get_previous_message_respects_partition() {
            let store = MockMessageStore::new();
            store.add_message(make_message("p1", "i1", 100, "p1 message"));
            store.add_message(make_message("p2", "i1", 200, "p2 message"));

            let prev = store.get_previous_message("p1", "i1", 300).await.unwrap();
            assert!(prev.is_some());
            assert_eq!(prev.unwrap().timestamp, 100); // Only p1 message
        }

        #[tokio::test]
        async fn get_previous_message_returns_none_for_first_message() {
            let store = MockMessageStore::new();
            store.add_message(make_message("p1", "i1", 100, "first"));

            let prev = store.get_previous_message("p1", "i1", 100).await.unwrap();
            assert!(prev.is_none());
        }

        #[tokio::test]
        async fn create_synapse_stores_connection() {
            let store = MockMessageStore::new();
            store.create_synapse(100, 200, "p1", "i1", 0.9).await.unwrap();

            assert!(store.has_synapse(100, 200));
            assert!(!store.has_synapse(200, 100)); // Direction matters
        }

        #[tokio::test]
        async fn get_thread_follows_synapse_chain() {
            let store = MockMessageStore::new();
            store.add_message(make_message("p1", "i1", 100, "first"));
            store.add_message(make_message("p1", "i1", 200, "second"));
            store.add_message(make_message("p1", "i1", 300, "third"));

            // Create chain: 100 -> 200 -> 300
            store.create_synapse(100, 200, "p1", "i1", 0.9).await.unwrap();
            store.create_synapse(200, 300, "p1", "i1", 0.9).await.unwrap();

            let thread = store.get_thread_from_latest("p1", "i1", 50).await.unwrap();
            assert_eq!(thread.len(), 3);
            assert_eq!(thread[0].timestamp, 100); // Chronological order
            assert_eq!(thread[1].timestamp, 200);
            assert_eq!(thread[2].timestamp, 300);
        }

        #[tokio::test]
        async fn get_thread_stops_at_broken_synapse() {
            let store = MockMessageStore::new();
            store.add_message(make_message("p1", "i1", 100, "first"));
            store.add_message(make_message("p1", "i1", 200, "second - unrelated"));
            store.add_message(make_message("p1", "i1", 300, "third"));
            store.add_message(make_message("p1", "i1", 400, "fourth"));

            // Chain breaks at 200: 100 (no synapse) 200 -> 300 -> 400
            store.create_synapse(200, 300, "p1", "i1", 0.9).await.unwrap();
            store.create_synapse(300, 400, "p1", "i1", 0.9).await.unwrap();

            let thread = store.get_thread_from_latest("p1", "i1", 50).await.unwrap();
            assert_eq!(thread.len(), 3); // Only 200, 300, 400
            assert_eq!(thread[0].timestamp, 200);
            assert_eq!(thread[1].timestamp, 300);
            assert_eq!(thread[2].timestamp, 400);
        }

        #[tokio::test]
        async fn get_thread_respects_max_count() {
            let store = MockMessageStore::new();
            store.add_message(make_message("p1", "i1", 100, "first"));
            store.add_message(make_message("p1", "i1", 200, "second"));
            store.add_message(make_message("p1", "i1", 300, "third"));

            store.create_synapse(100, 200, "p1", "i1", 0.9).await.unwrap();
            store.create_synapse(200, 300, "p1", "i1", 0.9).await.unwrap();

            let thread = store.get_thread_from_latest("p1", "i1", 2).await.unwrap();
            assert_eq!(thread.len(), 2);
        }

        #[tokio::test]
        async fn get_thread_empty_for_no_messages() {
            let store = MockMessageStore::new();
            let thread = store.get_thread_from_latest("p1", "i1", 50).await.unwrap();
            assert!(thread.is_empty());
        }

        #[tokio::test]
        async fn get_thread_isolates_partitions() {
            let store = MockMessageStore::new();
            store.add_message(make_message("p1", "i1", 100, "p1 first"));
            store.add_message(make_message("p1", "i1", 200, "p1 second"));
            store.add_message(make_message("p2", "i1", 150, "p2 message"));

            store.create_synapse(100, 200, "p1", "i1", 0.9).await.unwrap();

            let thread_p1 = store.get_thread_from_latest("p1", "i1", 50).await.unwrap();
            let thread_p2 = store.get_thread_from_latest("p2", "i1", 50).await.unwrap();

            assert_eq!(thread_p1.len(), 2);
            assert_eq!(thread_p2.len(), 1);
        }
    }
}
