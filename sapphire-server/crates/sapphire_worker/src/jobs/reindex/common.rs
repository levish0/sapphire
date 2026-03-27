// Common utilities for reindex jobs (logging only, no progress tracking)

/// Default batch size for reindex operations
pub const DEFAULT_BATCH_SIZE: u32 = 10_000;

/// Max concurrent storage downloads per batch (R2, etc.)
pub const STORAGE_DOWNLOAD_CONCURRENCY: usize = 50;
