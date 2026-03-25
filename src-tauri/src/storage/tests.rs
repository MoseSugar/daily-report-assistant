use super::{
    database::Database, migrations::run_migrations, settings_repository::SettingsRepository,
    task_repository::TaskRepository,
};
use crate::domain::task_entry::TaskEntry;

#[test]
fn initializes_database_and_migrations() {
    let temp_dir = std::env::temp_dir();
    let db_path = temp_dir.join(format!("dra-test-{}.sqlite3", uuid::Uuid::new_v4()));
    let database = Database::new(&db_path);

    database.initialize().expect("database should initialize");
    run_migrations(&database).expect("migrations should run");

    assert!(db_path.exists());
    std::fs::remove_file(db_path).expect("temp database should be removable");
}

#[test]
fn returns_default_settings_from_scaffold_repository() {
    let temp_dir = std::env::temp_dir();
    let db_path = temp_dir.join(format!("dra-settings-{}.sqlite3", uuid::Uuid::new_v4()));
    let database = Database::new(&db_path);
    database.initialize().expect("database should initialize");
    run_migrations(&database).expect("migrations should run");
    let repository = SettingsRepository::new(database);

    let settings = repository
        .get()
        .expect("default settings should be available");

    assert_eq!(settings.global_hotkey, "Ctrl+Shift+D");

    std::fs::remove_file(db_path).expect("temp database should be removable");
}

#[test]
fn inserts_and_lists_tasks_by_date() {
    let temp_dir = std::env::temp_dir();
    let db_path = temp_dir.join(format!("dra-task-repo-{}.sqlite3", uuid::Uuid::new_v4()));
    let database = Database::new(&db_path);
    database.initialize().expect("database should initialize");
    run_migrations(&database).expect("migrations should run");

    let repository = TaskRepository::new(database);
    let task = TaskEntry::new("验证仓储保存").expect("task should be created");
    let date = task.date.clone();

    repository.insert(&task).expect("task should be inserted");
    let tasks = repository
        .list_by_date(&date)
        .expect("tasks should be returned");

    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].content, "验证仓储保存");

    std::fs::remove_file(db_path).expect("temp database should be removable");
}

#[test]
fn saves_and_reads_settings_from_database() {
    let temp_dir = std::env::temp_dir();
    let db_path = temp_dir.join(format!(
        "dra-settings-save-{}.sqlite3",
        uuid::Uuid::new_v4()
    ));
    let database = Database::new(&db_path);
    database.initialize().expect("database should initialize");
    run_migrations(&database).expect("migrations should run");

    let repository = SettingsRepository::new(database);
    let mut settings = repository.get().expect("default settings should load");
    settings.off_work_time = "17:30".to_string();
    settings.remind_before_minutes = 10;
    settings.ai_system_prompt = "保持专业简洁".to_string();
    let saved = repository.save(&settings).expect("settings should save");
    let reloaded = repository.get().expect("settings should reload");

    assert_eq!(saved.off_work_time, "17:30");
    assert_eq!(reloaded.remind_before_minutes, 10);
    assert_eq!(reloaded.ai_system_prompt, "保持专业简洁");

    std::fs::remove_file(db_path).expect("temp database should be removable");
}
