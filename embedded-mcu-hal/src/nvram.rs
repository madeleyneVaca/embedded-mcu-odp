//! Traits for NVRAM (Non-Volatile Random Access Memory) storage and management.

/// An individual NVRAM storage cell.
pub trait NvramStorage<'a, T>: Send {
    /// Reads the value from the NVRAM storage cell.
    fn read(&self) -> T;

    /// Writes a value to the NVRAM storage cell.
    fn write(&mut self, value: T);
}

/// Trait for a collection of individually-addressable NVRAM storage cells.
/// StoredType is typically the word size of the platform CPU (e.g. u32).
pub trait Nvram<'a, StorageType, StoredType, const CELL_COUNT: usize>: Send
where
    StorageType: NvramStorage<'a, StoredType>,
{
    /// Returns an array of mutable storage cells.
    fn storage(&'a mut self) -> &'a mut [StorageType; CELL_COUNT];

    /// Dumps the contents of all storage cells into an array.
    /// This is for integrity validation purposes and not an alternative to `storage`.
    fn dump_storage(&self) -> [StoredType; CELL_COUNT];

    /// Clears all storage cells to their default state.
    fn clear_storage(&mut self);
}
