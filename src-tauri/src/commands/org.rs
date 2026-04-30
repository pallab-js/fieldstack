use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tauri::State;
use chrono::Utc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Company {
    pub id: String,
    pub name: String,
    pub logo_url: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Person {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub avatar_color: Option<String>,
    pub initials: String,
}

#[tauri::command]
pub async fn get_companies(pool: State<'_, SqlitePool>) -> Result<Vec<Company>, String> {
    sqlx::query_as::<_, Company>("SELECT id, name, logo_url, created_at FROM companies ORDER BY name")
        .fetch_all(&*pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_people(pool: State<'_, SqlitePool>) -> Result<Vec<Person>, String> {
    sqlx::query_as::<_, Person>("SELECT id, name, email, phone, avatar_color, initials FROM people ORDER BY name")
        .fetch_all(&*pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn seed_data(pool: State<'_, SqlitePool>) -> Result<(), String> {
    seed_data_inner(&pool).await
}

pub async fn seed_data_inner(pool: &SqlitePool) -> Result<(), String> {
    // Check if already seeded
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM companies")
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

    if count > 0 {
        return Ok(());
    }

    let now = Utc::now().to_rfc3339();

    // Seed companies
    let c1 = Uuid::new_v4().to_string();
    let c2 = Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO companies (id, name, created_at) VALUES (?, ?, ?), (?, ?, ?)")
        .bind(&c1).bind("Fieldstack Fertilizers").bind(&now)
        .bind(&c2).bind("Fieldstack Construction").bind(&now)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

    // Seed people
    let p1 = Uuid::new_v4().to_string();
    let p2 = Uuid::new_v4().to_string();
    let p3 = Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO people (id, name, email, initials, avatar_color, created_at) VALUES \
         (?, 'Gautam', 'gautam@fieldstack.in', 'GA', '#5e6ad2', ?), \
         (?, 'Ravi Kumar', 'ravi@fieldstack.in', 'RK', '#27a644', ?), \
         (?, 'Priya Singh', 'priya@fieldstack.in', 'PS', '#f59e0b', ?)"
    )
    .bind(&p1).bind(&now)
    .bind(&p2).bind(&now)
    .bind(&p3).bind(&now)
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;

    // Link people to companies
    sqlx::query(
        "INSERT INTO person_companies (person_id, company_id) VALUES \
         (?, ?), (?, ?), (?, ?), (?, ?)"
    )
    .bind(&p1).bind(&c1)
    .bind(&p1).bind(&c2)
    .bind(&p2).bind(&c1)
    .bind(&p3).bind(&c2)
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn create_company(
    pool: State<'_, SqlitePool>,
    name: String,
    logo_url: Option<String>,
) -> Result<String, String> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    
    sqlx::query("INSERT INTO companies (id, name, logo_url, created_at) VALUES (?, ?, ?, ?)")
        .bind(&id).bind(&name).bind(&logo_url).bind(&now)
        .execute(&*pool).await.map_err(|e| e.to_string())?;
    
    Ok(id)
}

#[tauri::command]
pub async fn update_company(
    pool: State<'_, SqlitePool>,
    id: String,
    name: String,
    logo_url: Option<String>,
) -> Result<(), String> {
    sqlx::query("UPDATE companies SET name = ?, logo_url = ? WHERE id = ?")
        .bind(&name).bind(&logo_url).bind(&id)
        .execute(&*pool).await.map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub async fn delete_company(pool: State<'_, SqlitePool>, id: String) -> Result<(), String> {
    sqlx::query("DELETE FROM companies WHERE id = ?")
        .bind(&id)
        .execute(&*pool).await.map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub async fn create_person(
    pool: State<'_, SqlitePool>,
    name: String,
    email: Option<String>,
    phone: Option<String>,
    company_ids: Vec<String>,
) -> Result<String, String> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    
    // Generate initials
    let initials: String = name.split_whitespace()
        .filter_map(|w| w.chars().next())
        .take(2)
        .collect::<String>()
        .to_uppercase();
    
    // Random avatar color
    let colors = ["#5e6ad2", "#27a644", "#f59e0b", "#ef4444", "#8b5cf6"];
    let avatar_color = colors[id.len() % colors.len()];
    
    sqlx::query("INSERT INTO people (id, name, email, phone, initials, avatar_color, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)")
        .bind(&id).bind(&name).bind(&email).bind(&phone).bind(&initials).bind(avatar_color).bind(&now)
        .execute(&*pool).await.map_err(|e| e.to_string())?;
    
    // Link to companies
    for company_id in company_ids {
        sqlx::query("INSERT INTO person_companies (person_id, company_id) VALUES (?, ?)")
            .bind(&id).bind(&company_id)
            .execute(&*pool).await.map_err(|e| e.to_string())?;
    }
    
    Ok(id)
}

#[tauri::command]
pub async fn update_person(
    pool: State<'_, SqlitePool>,
    id: String,
    name: String,
    email: Option<String>,
    phone: Option<String>,
    company_ids: Vec<String>,
) -> Result<(), String> {
    let initials: String = name.split_whitespace()
        .filter_map(|w| w.chars().next())
        .take(2)
        .collect::<String>()
        .to_uppercase();
    
    sqlx::query("UPDATE people SET name = ?, email = ?, phone = ?, initials = ? WHERE id = ?")
        .bind(&name).bind(&email).bind(&phone).bind(&initials).bind(&id)
        .execute(&*pool).await.map_err(|e| e.to_string())?;
    
    // Update company links
    sqlx::query("DELETE FROM person_companies WHERE person_id = ?")
        .bind(&id)
        .execute(&*pool).await.map_err(|e| e.to_string())?;
    
    for company_id in company_ids {
        sqlx::query("INSERT INTO person_companies (person_id, company_id) VALUES (?, ?)")
            .bind(&id).bind(&company_id)
            .execute(&*pool).await.map_err(|e| e.to_string())?;
    }
    
    Ok(())
}

#[tauri::command]
pub async fn delete_person(pool: State<'_, SqlitePool>, id: String) -> Result<(), String> {
    sqlx::query("DELETE FROM person_companies WHERE person_id = ?")
        .bind(&id)
        .execute(&*pool).await.map_err(|e| e.to_string())?;

    sqlx::query("DELETE FROM people WHERE id = ?")
        .bind(&id)
        .execute(&*pool).await.map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub async fn get_person_companies(pool: State<'_, SqlitePool>, person_id: String) -> Result<Vec<String>, String> {
    let company_ids: Vec<String> = sqlx::query_scalar("SELECT company_id FROM person_companies WHERE person_id = ?")
        .bind(&person_id)
        .fetch_all(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(company_ids)
}
