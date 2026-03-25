use crate::{domain::error::AppError, storage::database::Database};

const INITIAL_MIGRATION_SQL: &str = include_str!("../../migrations/0001_initial.sql");
const ADD_AI_SYSTEM_PROMPT_SQL: &str =
    include_str!("../../migrations/0002_add_ai_system_prompt.sql");

pub fn run_migrations(database: &Database) -> Result<(), AppError> {
    let connection = database.open()?;
    connection
        .execute_batch(INITIAL_MIGRATION_SQL)
        .map_err(|error| AppError::database(error.to_string()))?;

    let _ = connection.execute_batch(ADD_AI_SYSTEM_PROMPT_SQL);

    Ok(())
}
