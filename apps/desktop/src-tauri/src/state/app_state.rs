use crate::services::app_info_service::AppInfoService;
use crate::services::window_service::WindowService;

#[derive(Debug, Default)]
pub struct AppState {
    pub app_info_service: AppInfoService,
    pub window_service: WindowService,
}
