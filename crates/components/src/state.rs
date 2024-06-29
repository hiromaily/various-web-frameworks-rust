use crate::usecases::{admin, app, auth};
use std::sync::Arc;

pub struct GlobalState {
    pub app_name: String,
}

pub struct AuthState {
    pub auth_usecase: Arc<dyn auth::AuthUsecase>,
}

pub struct AdminState {
    pub admin_usecase: Arc<dyn admin::AdminUsecase>,
}

pub struct AppState {
    pub app_usecase: Arc<dyn app::AppUsecase>,
}
