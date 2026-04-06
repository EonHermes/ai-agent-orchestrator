#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::db::tasks::{create, delete, get, list, update};
    use crate::models::{CreateTask, TaskStatus, TaskFilter, UpdateTask};
    use sqlx::{Pool, Postgres};
    use std::time::Duration;
    use tokio::time::sleep;

    // Helper to create a test database pool
    async fn setup_test_db() -> Pool<Postgres> {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/task_dashboard".to_string());

        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .connect(&database_url)
            .await
            .expect("Failed to connect to database");

        // Clean before tests
        sqlx::query!("DELETE FROM tasks").execute(&pool).await.ok();

        pool
    }

    #[tokio::test]
    async fn test_create_task() {
        let pool = setup_test_db().await;

        let payload = CreateTask {
            title: "Test task".to_string(),
            description: Some("Test description".to_string()),
            status: TaskStatus::Todo,
            priority: 5,
            tags: vec!["test".to_string(), "rust".to_string()],
        };

        let task = create(&pool, payload).await.expect("Failed to create task");

        assert_eq!(task.title, "Test task");
        assert_eq!(task.priority, 5);
        assert_eq!(task.tags.len(), 2);
        assert_eq!(task.status, TaskStatus::Todo);
    }

    #[tokio::test]
    async fn test_get_task() {
        let pool = setup_test_db().await;

        let payload = CreateTask {
            title: "Get test".to_string(),
            description: None,
            status: TaskStatus::InProgress,
            priority: 3,
            tags: vec![],
        };

        let created = create(&pool, payload).await.expect("Failed to create task");

        let fetched = get(&pool, created.id).await.expect("Failed to get task");
        assert!(fetched.is_some());
        assert_eq!(fetched.unwrap().id, created.id);
    }

    #[tokio::test]
    async fn test_update_task() {
        let pool = setup_test_db().await;

        let payload = CreateTask {
            title: "Original".to_string(),
            description: None,
            status: TaskStatus::Todo,
            priority: 1,
            tags: vec!["a".to_string()],
        };

        let created = create(&pool, payload).await.expect("Failed to create task");

        let update = UpdateTask {
            title: Some("Updated".to_string()),
            priority: Some(9),
            status: Some(TaskStatus::Done),
            tags: Some(vec!["b".to_string()]),
            description: None,
        };

        let updated = update(&pool, created.id, update).await.expect("Failed to update task");

        assert_eq!(updated.title, "Updated");
        assert_eq!(updated.priority, 9);
        assert_eq!(updated.status, TaskStatus::Done);
        assert_eq!(updated.tags, vec!["b".to_string()]);
    }

    #[tokio::test]
    async fn test_delete_task() {
        let pool = setup_test_db().await;

        let payload = CreateTask {
            title: "Delete me".to_string(),
            description: None,
            status: TaskStatus::Todo,
            priority: 0,
            tags: vec![],
        };

        let created = create(&pool, payload).await.expect("Failed to create task");

        delete(&pool, created.id).await.expect("Failed to delete task");

        let fetched = get(&pool, created.id).await.expect("Failed to get deleted task");
        assert!(fetched.is_none());
    }

    #[tokio::test]
    async fn test_list_tasks_with_filter() {
        let pool = setup_test_db().await;

        // Create multiple tasks
        let tasks = vec![
            CreateTask { title: "High priority".to_string(), priority: 10, status: TaskStatus::Todo, description: None, tags: vec!["work".to_string()] },
            CreateTask { title: "Medium priority".to_string(), priority: 5, status: TaskStatus::InProgress, description: None, tags: vec!["personal".to_string()] },
            CreateTask { title: "Low priority".to_string(), priority: 1, status: TaskStatus::Done, description: None, tags: vec!["work".to_string()] },
        ];

        for task in tasks {
            create(&pool, task).await.ok();
        }

        sleep(Duration::from_millis(100)).await;

        // Filter by status
        let todo_tasks = list(&pool, crate::models::TaskFilter { status: Some(TaskStatus::Todo), priority_min: None, priority_max: None, tags: vec![], search: None }).await.expect("Failed to list");
        assert_eq!(todo_tasks.len(), 1);
        assert_eq!(todo_tasks[0].title, "High priority");

        // Filter by tags
        let work_tasks = list(&pool, crate::models::TaskFilter { status: None, priority_min: None, priority_max: None, tags: vec!["work".to_string()], search: None }).await.expect("Failed to list");
        assert_eq!(work_tasks.len(), 2);
    }
}
