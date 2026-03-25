use crate::{
    domain::{
        error::AppError,
        task_entry::{TaskEntry, TaskStatus},
    },
    storage::task_repository::TaskRepository,
};
use chrono::Local;

#[derive(Debug, Clone)]
pub struct TaskService {
    repository: TaskRepository,
}

impl TaskService {
    pub fn new(repository: TaskRepository) -> Self {
        Self { repository }
    }

    pub fn create_task(&self, content: impl Into<String>) -> Result<TaskEntry, AppError> {
        let task = TaskEntry::new(content)?;
        self.repository.insert(&task)?;
        Ok(task)
    }

    pub fn list_today_tasks(&self) -> Result<Vec<TaskEntry>, AppError> {
        let today = Local::now().format("%Y-%m-%d").to_string();
        self.repository.list_by_date(&today)
    }

    pub fn update_task(
        &self,
        task_id: String,
        status: TaskStatus,
        note: String,
    ) -> Result<TaskEntry, AppError> {
        self.repository.update_task(&task_id, status, note)
    }
}

#[cfg(test)]
mod tests {
    use super::TaskService;
    use crate::storage::{
        database::Database, migrations::run_migrations, task_repository::TaskRepository,
    };

    #[test]
    fn creates_task_with_default_status_and_persists_it() {
        let temp_dir = std::env::temp_dir();
        let db_path = temp_dir.join(format!("dra-task-service-{}.sqlite3", uuid::Uuid::new_v4()));
        let database = Database::new(&db_path);
        database.initialize().expect("database should initialize");
        run_migrations(&database).expect("migrations should run");

        let repository = TaskRepository::new(database.clone());
        let service = TaskService::new(repository);

        let task = service
            .create_task("验证任务新增")
            .expect("task should be created");

        assert_eq!(task.status_as_str(), "done");
        assert_eq!(task.content, "验证任务新增");

        std::fs::remove_file(db_path).expect("temp database should be removable");
    }
}
