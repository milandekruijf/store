use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

/// A stored value.
///
/// # Example
///
/// ```
/// use store::Stored;
///
/// let mut stored = Stored::new("data", "Hello, world!")?;
///
/// stored.save()?;
/// ```
pub struct Stored<T>
where
    for<'de> T: Serialize + Deserialize<'de>,
{
    /// The path to the stored value.
    pub(super) path: PathBuf,
    /// The stored value.
    pub(super) value: T,
}

impl<T> Stored<T>
where
    for<'de> T: Serialize + Deserialize<'de>,
{
    /// Create a new stored value.
    pub fn new<P>(path: P, default: T) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref().with_extension("json");
        let file = File::create(&path)?;
        let value: T = serde_json::from_reader(file).unwrap_or(default);

        Ok(Self { path, value })
    }

    /// Save the stored value.
    pub fn save(&self) -> Result<()>
    {
        let file = File::create(&self.path)?;
        serde_json::to_writer(file, &self.value)?;
        Ok(())
    }

    /// Store a new value.
    pub fn store(&mut self, value: T) -> Result<()>
    {
        self.value = value;
        self.save()?;
        Ok(())
    }

    /// Delete the file.
    pub fn delete(&self) -> Result<()>
    {
        fs::remove_file(&self.path)?;
        Ok(())
    }

    /// Get the stored value.
    pub fn value(&self) -> &T
    {
        &self.value
    }
}
