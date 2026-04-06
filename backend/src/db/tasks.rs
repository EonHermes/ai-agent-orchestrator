use sqlx::{PgPool, postgres::PgQueryResult};
use uuid::Uuid;
use chrono::Utc;
use crate::{error::Result, models::{Task, CreateTask, UpdateTask, TaskFilter, TaskStatus}};

pub async fn list(pool: &PgPool, filter: TaskFilter) -> Result<Vec<Task>> {
    let mut query = "
        SELECT id, title, description, status as \"status: TaskStatus\", priority, tags, created_at, updated_at
        FROM tasks
        WHERE 1=1
    ".to_string();

    let mut conditions = Vec::new();

    if let Some(status) = &filter.status {
        conditions.push(format!("status = ${}", conditions.len() + 2));
        query = format!("{} AND {}", query, conditions.join(" AND "));
    }

    if let Some(min) = filter.priority_min {
        conditions.push(format!("priority >= ${}", conditions.len() + 2));
        query = format!("{} AND {}", query, conditions.join(" AND "));
    }

    if let Some(max) = filter.priority_max {
        conditions.push(format!("priority <= ${}", conditions.len() + 2));
        query = format!("{} AND {}", query, conditions.join(" AND "));
    }

    if !filter.tags.is_empty() {
        for (i, tag) in filter.tags.iter().enumerate() {
            conditions.push(format!("${} = ANY(tags)", i + 2));
        }
        query = format!("{} AND {}", query, conditions.join(" AND "));
    }

    if let Some(search) = &filter.search {
        conditions.push("(title ILIKE $2 OR description ILIKE $2)".to_string());
        query = format!("{} AND {}", query, conditions.join(" AND "));
    }

    query = format!("{} ORDER BY priority DESC, created_at DESC", query);

    let mut qb = sqlx::query_as::<_, Task>(&query);

    let mut idx = 1;
    if let Some(status) = &filter.status {
        qb = qb.bind(status.clone() as TaskStatus);
        idx += 1;
    }

    if let Some(min) = filter.priority_min {
        qb = qb.bind(min);
        idx += 1;
    }

    if let Some(max) = filter.priority_max {
        qb = qb.bind(max);
        idx += 1;
    }

    if !filter.tags.is_empty() {
        for tag in filter.tags {
            qb = qb.bind(tag);
        }
    }

    if let Some(search) = &filter.search {
        let pattern = format!("%{}%", search);
        qb = qb.bind(pattern);
    }

    let tasks = qb.fetch_all(pool).await?;
    Ok(tasks)
}

pub async fn get(pool: &PgPool, id: Uuid) -> Result<Option<Task>> {
    let task = sqlx::query_as!(
        Task,
        "
        SELECT id, title, description, status as \"status: TaskStatus\", priority, tags, created_at, updated_at
        FROM tasks
        WHERE id = $1
        ",
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(task)
}

pub async fn create(pool: &PgPool, payload: CreateTask) -> Result<Task> {
    let now = Utc::now();

    let task = sqlx::query_as!(
        Task,
        "
        INSERT INTO tasks (id, title, description, status, priority, tags, created_at, updated_at)
        VALUES ($1, $2, $3, $4 as \"TaskStatus\", $5, $6, $7, $8)
        RETURNING id, title, description, status as \"status: TaskStatus\", priority, tags, created_at, updated_at
        ",
        Uuid::new_v4(),
        payload.title,
        payload.description,
        payload.status as TaskStatus,
        payload.priority,
        payload.tags,
        now,
        now
    )
    .fetch_one(pool)
    .await?;

    Ok(task)
}

pub async fn update(
    pool: &PgPool,
    id: Uuid,
    payload: UpdateTask,
) -> Result<Task> {
    let mut updates = Vec::new();
    let mut idx = 1;

    if let Some(title) = &payload.title {
        updates.push(format!("title = ${}", idx));
        idx += 1;
    }

    if let Some(description) = &payload.description {
        updates.push(format!("description = ${}", idx));
        idx += 1;
    }

    if let Some(status) = &payload.status {
        updates.push(format!("status = ${} as \"TaskStatus\"", idx));
        idx += 1;
    }

    if let Some(priority) = &payload.priority {
        updates.push(format!("priority = ${}", idx));
        idx += 1;
    }

    if let Some(tags) = &payload.tags {
        updates.push(format!("tags = ${}", idx));
        idx += 1;
    }

    updates.push(format!("updated_at = ${}", idx));
    let query = format!(
        "UPDATE tasks SET {} WHERE id = ${} RETURNING id, title, description, status as \"status: TaskStatus\", priority, tags, created_at, updated_at",
        updates.join(", "),
        idx + 1
    );

    let mut qb = sqlx::query_as::<_, Task>(&query);

    idx = 1;
    if let Some(title) = &payload.title {
        qb = qb.bind(title);
        idx += 1;
    }
    if let Some(description) = &payload.description {
        qb = qb.bind(description);
        idx += 1;
    }
    if let Some(status) = &payload.status {
        qb = qb.bind(*status as TaskStatus);
        idx += 1;
    }
    if let Some(priority) = &payload.priority {
        qb = qb.bind(priority);
        idx += 1;
    }
    if let Some(tags) = &payload.tags {
        qb = qb.bind(tags);
        idx += 1;
    }
    qb = qb.bind(Utc::now());
    qb = qb.bind(id);

    let task = qb.fetch_one(pool).await?;
    Ok(task)
}

pub async fn delete(pool: &PgPool, id: Uuid) -> Result<()> {
    let result: PgQueryResult = sqlx::query!("DELETE FROM tasks WHERE id = $1", id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(crate::error::Error::NotFound(format!("Task {} not found", id)));
    }

    Ok(())
}
