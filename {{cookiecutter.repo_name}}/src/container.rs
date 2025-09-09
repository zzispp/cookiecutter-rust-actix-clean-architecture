use crate::config::AppConfig;
use crate::domain::repositories::todo::TodoRepository;
use crate::domain::services::service_context::ServiceContextService;
use crate::domain::services::todo::TodoService;
use crate::infrastructure::databases::postgresql::db_pool;
use crate::infrastructure::repositories::todo::TodoDieselRepository;
use crate::infrastructure::services::service_context::ServiceContextServiceImpl;
use crate::services::todo::TodoServiceImpl;
use std::sync::Arc;

pub struct Container {
    pub todo_service: Arc<dyn TodoService>,
    pub service_context_service: Arc<dyn ServiceContextService>,
}

impl Container {
    pub fn new(config: &AppConfig) -> Self {
        let pool = Arc::new(db_pool(config));
        let todo_repository: Arc<dyn TodoRepository> =
            Arc::new(TodoDieselRepository::new(pool.clone()));
        let todo_service = Arc::new(TodoServiceImpl {
            repository: todo_repository,
        });
        let service_context_service = Arc::new(ServiceContextServiceImpl::new(pool.clone()));
        Container {
            todo_service,
            service_context_service,
        }
    }
}
