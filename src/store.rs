use super::Stored;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

/// A store for storing data.
///
/// # Example
///
/// ```
/// use store::Store;
///
/// let mut store = Store::new("data");
///
/// store.save("test", "Hello, world!")?;
/// ```
pub struct Store<T>
where
    for<'de> T: Serialize + Deserialize<'de>,
{
    /// The path to the store.
    pub(super) path: PathBuf,
    /// The data stored in the store.
    pub(super) data: HashMap<PathBuf, Stored<T>>,
}

impl<T> Store<T>
where
    for<'de> T: Serialize + Deserialize<'de>,
{
    /// Create a new store.
    pub fn new<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref().to_path_buf();

        fs::create_dir_all(&path)?;

        Ok(Self {
            path,
            data: HashMap::new(),
        })
    }

    /// Get all data from the store.
    pub fn all(&self) -> Vec<&T>
    {
        fs::read_dir(&self.path)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.extension().unwrap() == "json")
            .map(|path| self.data.get(&path).unwrap().value())
            .collect()
    }

    /// Save data to the store.
    pub fn save(&mut self, name: &str, value: T) -> Result<()>
    {
        let path = self.path.join(name);

        self.data
            .entry(path.clone())
            .or_insert_with(|| Stored::new(path, value).unwrap())
            .save()?;

        Ok(())
    }

    /// Get data from the store.
    pub fn get(&self, name: &str) -> Option<&T>
    {
        let path = self.path.join(name);

        self.data.get(&path).map(|stored| stored.value())
    }

    /// Delete data from the store.
    pub fn delete(&mut self, name: &str) -> Result<()>
    {
        let path = self.path.join(name);

        self.data.remove(&path).unwrap().delete()?;

        if self.all().is_empty()
        {
            fs::remove_dir_all(&self.path)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    /// Test that the store can save a value.
    #[test]
    fn test_store() -> Result<()>
    {
        let mut store: Store<String> = Store::new("test")?;

        let name = "hello.json";
        let value = String::from("world");

        store.save(name, value.clone())?;

        assert_eq!(store.get(name), Some(&value));

        store.delete(name)?;

        Ok(())
    }

    /// Test that the store can save multiple values.
    #[test]
    fn test_all() -> Result<()>
    {
        let mut store: Store<String> = Store::new("test")?;

        let entries = vec![
            ("hello.json", String::from("world")),
            ("goodbye.json", String::from("world")),
        ];

        for (name, value) in entries.iter()
        {
            store.save(name, value.clone())?;
        }

        let stored = store.all();
        let compare = entries.iter().map(|(_, value)| value).collect::<Vec<_>>();

        assert_eq!(stored, compare);

        for (name, _) in entries.iter()
        {
            store.delete(name)?;
        }

        Ok(())
    }
}
