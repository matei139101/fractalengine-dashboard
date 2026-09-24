use std::{rc::Rc};
use fractalengine::{ServiceRegistry, LifeCycleHook, Service};

pub struct DashboardService {
    name: String,
}

impl DashboardService {
    pub fn new() -> Rc<Self> {
        let dashboard_service = Self {
            name: "DashboardService".to_string(),
        };

        Rc::new(dashboard_service)
    }

    pub fn register(self_ptr: Rc<Self>, service_registry: &mut ServiceRegistry) {
        service_registry.register_service(self_ptr.clone());
        service_registry.register_lifecycled_service(self_ptr.clone());
    }
}

impl Service for DashboardService {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl LifeCycleHook for DashboardService {
    fn init(&self) {
        println!("Initialization of DashboardService");
    }
    fn update(&self) {
        println!("Update from DashboardService");
    }
}
