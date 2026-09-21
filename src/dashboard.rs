use std::sync::Arc;

use fractalengine::ServiceLocator;
use fractalengine_core::{Lifecycled, Service};

pub struct DashboardService {
    name: String,
}

impl DashboardService {
    pub fn new() -> Arc<Self> {
        let dashboard_service = Self {
            name: "DashboardService".to_string(),
        };

        Arc::new(dashboard_service)
    }

    pub fn register(self_ptr: Arc<Self>, service_locator: &mut ServiceLocator) {
        service_locator.register_service(self_ptr.clone());
        service_locator.register_lifcycled_service(self_ptr.clone());
    }
}

impl Service for DashboardService {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Lifecycled for DashboardService {
    fn init(&self) {}
    fn re_init(&self) {}
    fn tick(&self) {}
    fn update(&self) {}
    fn cleanup(&self) {}
}
