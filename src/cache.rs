use crate::types::LocationCache;
use anyhow::Result;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub fn get_config_dir() -> Result<PathBuf> {
    let config_dir = if let Some(dir) = dirs::config_dir() {
        dir.join("weather_cli")
    } else {
        return Err(anyhow::anyhow!(
            "Could not determine config directory"
        ));
    };

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }

    Ok(config_dir)
}

pub fn get_cache_path() -> Result<PathBuf> {
    Ok(get_config_dir()?.join("config"))
}

pub fn read_cache() -> Result<Option<LocationCache>> {
    let path = get_cache_path()?;
    read_cache_from_path(&path)
}

pub fn write_cache(cache: &LocationCache) -> Result<()> {
    let path = get_cache_path()?;
    write_cache_to_path(&path, cache)
}

pub fn read_cache_from_path(path: &Path) -> Result<Option<LocationCache>> {
    if !path.exists() {
        return Ok(None);
    }

    let contents = fs::read_to_string(path)?;
    let cache = serde_json::from_str::<LocationCache>(&contents)?;
    Ok(Some(cache))
}

pub fn write_cache_to_path(path: &Path, cache: &LocationCache) -> Result<()> {
    let contents = serde_json::to_string_pretty(cache)?;
    fs::write(path, contents)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_missing_cache_returns_none() {
        let dir = tempfile::tempdir().expect("tempdir should be created");
        let path = dir.path().join("config");

        let value = read_cache_from_path(&path).expect("reading missing cache should succeed");
        assert!(value.is_none());
    }

    #[test]
    fn write_and_read_cache_roundtrip() {
        let dir = tempfile::tempdir().expect("tempdir should be created");
        let path = dir.path().join("config");
        let cache = LocationCache {
            ip: "8.8.8.8".to_string(),
            lat: 37.386,
            lon: -122.084,
        };

        write_cache_to_path(&path, &cache).expect("cache should be written");
        let loaded = read_cache_from_path(&path)
            .expect("cache should be read")
            .expect("cache should be present");

        assert_eq!(loaded.ip, cache.ip);
        assert_eq!(loaded.lat, cache.lat);
        assert_eq!(loaded.lon, cache.lon);
    }

    #[test]
    fn read_invalid_cache_returns_error() {
        let dir = tempfile::tempdir().expect("tempdir should be created");
        let path = dir.path().join("config");
        fs::write(&path, "not-json").expect("invalid content should be written");

        let err = read_cache_from_path(&path).expect_err("invalid json should error");
        assert!(err.to_string().contains("expected"));
    }
}
