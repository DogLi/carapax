use crate::store::SessionStore;
use serde::{de::DeserializeOwned, Serialize};
use std::{fmt, sync::Arc, time::Duration};
use tokio::sync::Mutex;

/// Actual session available in context
#[derive(Clone)]
pub struct Session<S> {
    namespace: String,
    store: Arc<Mutex<S>>,
}

impl<S> Session<S>
where
    S: SessionStore,
{
    pub(crate) fn new<N: Into<String>>(namespace: N, store: Arc<Mutex<S>>) -> Self {
        Self {
            namespace: namespace.into(),
            store,
        }
    }

    fn build_key(&self, key: &str) -> SessionKey {
        SessionKey::new(self.namespace.clone(), key)
    }

    /// Get value of key
    ///
    /// If key not exists, None is returned
    pub async fn get<O>(&mut self, key: &str) -> Result<Option<O>, S::Error>
    where
        O: DeserializeOwned + Send + Sync,
    {
        self.store.lock().await.get(self.build_key(key)).await
    }

    /// Set key to hold the given value
    pub async fn set<I>(&mut self, key: &str, val: &I) -> Result<(), S::Error>
    where
        I: Serialize + Send + Sync,
    {
        self.store.lock().await.set(self.build_key(key), val).await
    }

    /// Set a timeout on key
    ///
    /// After the timeout has expired, the key will automatically be deleted
    pub async fn expire(&mut self, key: &str, seconds: usize) -> Result<(), S::Error> {
        self.store.lock().await.expire(self.build_key(key), seconds).await
    }

    /// Remove the specified key
    pub async fn del(&mut self, key: &str) -> Result<(), S::Error> {
        self.store.lock().await.del(self.build_key(key)).await
    }
}

/// A session key used in store
#[derive(Debug, Clone)]
pub struct SessionKey {
    namespace: String,
    name: String,
}

impl SessionKey {
    fn new<A, B>(namespace: A, name: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            namespace: namespace.into(),
            name: name.into(),
        }
    }

    /// Namespace for a key
    ///
    /// Format: `(user-id|chat-id)-(user-id|chat-id)`
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Key name
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for SessionKey {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(out, "{}-{}", self.namespace, self.name)
    }
}

/// Defines a lifetime for each session
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SessionLifetime {
    /// Session will live forever
    ///
    /// Default variant
    Forever,
    /// Session will expire at given duration
    Duration(Duration),
}

impl Default for SessionLifetime {
    fn default() -> Self {
        SessionLifetime::Forever
    }
}

impl From<Duration> for SessionLifetime {
    fn from(duration: Duration) -> Self {
        SessionLifetime::Duration(duration)
    }
}

impl From<u64> for SessionLifetime {
    fn from(seconds: u64) -> Self {
        SessionLifetime::Duration(Duration::from_secs(seconds))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::future;
    use std::{collections::HashMap, sync::Mutex};

    #[derive(Default)]
    struct Store {
        data: Mutex<HashMap<String, String>>,
        expire_calls: Mutex<Vec<(String, usize)>>,
    }

    impl SessionStore for Store {
        fn get<O>(&self, key: SessionKey) -> Box<dyn Future<Item = Option<O>, Error = Error> + Send>
        where
            O: DeserializeOwned + Send + 'static,
        {
            match self.data.lock().unwrap().get(&key.to_string()) {
                Some(x) => Box::new(future::result(serde_json::from_str(&x).map(Some)).from_err()),
                None => Box::new(future::ok(None)),
            }
        }

        fn set<I>(&self, key: SessionKey, val: &I) -> Box<dyn Future<Item = (), Error = Error> + Send>
        where
            I: Serialize,
        {
            Box::new(
                future::result(serde_json::to_string(val).and_then(|val| {
                    self.data.lock().unwrap().insert(key.to_string(), val);
                    Ok(())
                }))
                .from_err(),
            )
        }

        fn expire(&self, key: SessionKey, seconds: usize) -> Box<dyn Future<Item = (), Error = Error> + Send> {
            self.expire_calls.lock().unwrap().push((key.to_string(), seconds));
            Box::new(future::ok(()))
        }

        fn del(&self, key: SessionKey) -> Box<dyn Future<Item = (), Error = Error> + Send> {
            self.data.lock().unwrap().remove(&key.to_string());
            Box::new(future::ok(()))
        }
    }

    #[test]
    fn session() {
        let store = Arc::new(Store::default());
        let session = Session::new("namespace", store.clone());
        session.set("key", &1).wait().unwrap();
        assert_eq!(session.get::<usize>("key").wait().unwrap().unwrap(), 1);
        session.expire("key", 10).wait().unwrap();
        assert!(store
            .expire_calls
            .lock()
            .unwrap()
            .contains(&(String::from("namespace-key"), 10)));
        session.del("key").wait().unwrap();
        assert!(session.get::<usize>("key").wait().unwrap().is_none());
    }

    #[test]
    fn session_key() {
        let key = SessionKey::new("namespace", "name");
        assert_eq!(key.namespace(), "namespace");
        assert_eq!(key.name(), "name");
        assert_eq!(key.to_string(), "namespace-name");
    }

    #[test]
    fn session_lifetime() {
        assert_eq!(SessionLifetime::default(), SessionLifetime::Forever);
        assert_eq!(
            SessionLifetime::from(Duration::from_secs(1)),
            SessionLifetime::Duration(Duration::from_secs(1)),
        );
        assert_eq!(
            SessionLifetime::from(1),
            SessionLifetime::Duration(Duration::from_secs(1)),
        );
    }

    #[test]
    fn get_namespace_from_update() {
        assert_eq!(
            namespace_from_update(
                &serde_json::from_value(serde_json::json!({
                    "update_id": 1,
                    "message": {
                        "message_id": 1,
                        "date": 0,
                        "from": {"id": 1, "is_bot": false, "first_name": "test", "username": "username1"},
                        "chat": {"id": 1, "type": "private", "first_name": "test", "username": "username1"},
                        "text": "test middleware"
                    }
                }))
                .unwrap()
            ),
            "1-1"
        );

        assert_eq!(
            namespace_from_update(
                &serde_json::from_value(serde_json::json!({
                    "update_id": 1,
                    "inline_query": {
                        "id": "query id",
                        "from": {
                            "id": 1111,
                            "first_name": "Test Firstname",
                            "is_bot": false
                        },
                        "query": "query text",
                        "offset": "query offset"
                    }
                }))
                .unwrap()
            ),
            "1111-1111"
        );

        assert_eq!(
            namespace_from_update(
                &serde_json::from_value(serde_json::json!({
                    "update_id": 1,
                    "channel_post": {
                        "message_id": 1111,
                        "date": 0,
                        "author_signature": "test",
                        "chat": {
                            "id": 1,
                            "type": "channel",
                            "title": "channeltitle",
                            "username": "channelusername"
                        },
                        "text": "test message from channel"
                    }
                }))
                .unwrap()
            ),
            "1-1"
        );
    }
}
