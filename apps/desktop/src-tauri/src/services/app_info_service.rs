use crate::error::app_error::AppResult;

#[derive(Debug, Clone)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub os: String,
    pub arch: String,
}

#[derive(Debug, Clone)]
pub struct HealthCheck {
    pub status: String,
}

#[derive(Debug, Default)]
pub struct AppInfoService;

impl AppInfoService {
    pub fn get_app_info(&self) -> AppResult<AppInfo> {
        Ok(AppInfo {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
        })
    }

    pub fn health_check(&self) -> AppResult<HealthCheck> {
        Ok(HealthCheck {
            status: "ok".to_string(),
        })
    }
}
