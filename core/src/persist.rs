/// Persist defines standard methods for persisting data
pub trait Persist: Send {
    /// Standard method for read operations
    fn read(&self) -> Result<&[u8], std::io::Error>;
    /// Standard method for write operations
    fn write(&mut self, content: &[u8]) -> Result<(), std::io::Error>;
    /// Standard method for delete operations
    fn delete(&mut self) -> Result<(), std::io::Error>;
}
