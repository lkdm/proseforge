use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::hash::Hash;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct InMemoryPathStore<T>(Arc<Mutex<BTreeMap<T, PathBuf>>>);

impl<T> InMemoryPathStore<T>
where
    T: Ord + Clone + Hash,
{
    pub fn new() -> Self {
        InMemoryPathStore(Arc::new(Mutex::new(BTreeMap::new())))
    }

    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<PathBuf>
    where
        T: Borrow<Q>,
        Q: Ord + Hash,
    {
        self.0.lock().ok().and_then(|guard| guard.get(key).cloned())
    }

    pub fn insert(&self, key: T, path: PathBuf) -> Option<PathBuf> {
        self.0
            .lock()
            .ok()
            .and_then(|mut guard| guard.insert(key, path))
    }

    pub fn remove<Q: ?Sized>(&self, key: &Q) -> Option<PathBuf>
    where
        T: Borrow<Q>,
        Q: Ord + Hash,
    {
        self.0.lock().ok().and_then(|mut guard| guard.remove(key))
    }

    pub fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: Ord + Hash,
    {
        self.0
            .lock()
            .map(|guard| guard.contains_key(key))
            .unwrap_or(false)
    }

    pub fn len(&self) -> usize {
        self.0.lock().map(|guard| guard.len()).unwrap_or(0)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

// Implement Default if you want to be able to create an empty InMemoryPathStore easily
impl<T> Default for InMemoryPathStore<T>
where
    T: Ord + Clone + Hash,
{
    fn default() -> Self {
        Self::new()
    }
}
