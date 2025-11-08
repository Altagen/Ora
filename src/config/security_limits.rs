/// Security limits for Ora package manager
///
/// These limits prevent various DoS attacks and resource exhaustion.
///
/// Maximum size of a single extracted file (1 GB)
/// Prevents zip bombs and memory exhaustion
pub const MAX_EXTRACTED_FILE_SIZE: u64 = 1024 * 1024 * 1024; // 1 GB

/// Maximum total size of all extracted files (5 GB)
/// Prevents disk exhaustion attacks
pub const MAX_TOTAL_EXTRACTED_SIZE: u64 = 5 * 1024 * 1024 * 1024; // 5 GB

/// Maximum number of files in an archive (100,000)
/// Prevents billion laughs / zip bomb attacks
pub const MAX_FILES_IN_ARCHIVE: usize = 100_000;

/// Maximum depth of directory nesting (50 levels)
/// Prevents stack exhaustion and filesystem issues
pub const MAX_DIRECTORY_DEPTH: usize = 50;

/// Maximum size of downloaded archive (2 GB)
/// Prevents network DoS and disk exhaustion
pub const MAX_DOWNLOAD_SIZE: u64 = 2 * 1024 * 1024 * 1024; // 2 GB

/// Maximum path length (4096 bytes on most Unix systems)
/// Prevents path length DoS attacks
pub const MAX_PATH_LENGTH: usize = 4096;

/// Compression ratio warning threshold (100:1)
/// If decompressed size / compressed size > this, warn about potential zip bomb
/// Reserved for future use when compression ratio checking is implemented.
#[allow(dead_code)]
pub const COMPRESSION_RATIO_WARNING: u64 = 100;
