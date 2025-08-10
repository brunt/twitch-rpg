use assets_manager::AssetCache;
use std::sync::LazyLock;

// global const to read items and spells from ron files
pub static ASSETS: LazyLock<AssetCache> = LazyLock::new(|| AssetCache::new("server").unwrap());
