use crate::usecases::{admin, app, auth};
use std::sync::Arc;

#[derive(Clone)]
pub struct GlobalState {
    pub app_name: String,
}

#[derive(Clone)]
pub struct AuthState {
    pub auth_usecase: Arc<dyn auth::AuthUsecase>,
}

#[derive(Clone)]
pub struct AdminState {
    pub admin_usecase: Arc<dyn admin::AdminUsecase>,
}

#[derive(Clone)]
pub struct AppState {
    pub app_usecase: Arc<dyn app::AppUsecase>,
}
