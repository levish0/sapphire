//! R2 storage key prefixes and image size limits

/// Prefix for user profile/banner images
pub const USER_IMAGES_PREFIX: &str = "user-images";

/// Maximum size for profile images (4MB)
pub const PROFILE_IMAGE_MAX_SIZE: usize = 4 * 1024 * 1024;

/// Maximum size for banner images (8MB)
pub const BANNER_IMAGE_MAX_SIZE: usize = 8 * 1024 * 1024;

/// Maximum image dimensions (width, height) for processing
pub const IMAGE_MAX_WIDTH: u32 = 2000;
/// Constant value for image max height.
pub const IMAGE_MAX_HEIGHT: u32 = 2000;

/// Default image compression quality (0-100)
pub const IMAGE_COMPRESSION_QUALITY: u8 = 90;

/// Generate storage key for user images (profile/banner)
pub fn user_image_key(hash: &str, extension: &str) -> String {
    format!("{}/{}.{}", USER_IMAGES_PREFIX, hash, extension)
}
