use sqlx::SqlitePool;
use tauri::State;
use crate::models::setting::{Setting, UpdateSetting};

#[tauri::command]
pub async fn get_settings(pool: State<'_, SqlitePool>) -> Result<Setting, String> {
    sqlx::query_as::<_, Setting>("SELECT * FROM settings WHERE id = 1")
        .fetch_one(&*pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_settings(
    setting: UpdateSetting,
    pool: State<'_, SqlitePool>,
) -> Result<(), String> {
    sqlx::query(
        r#"
        UPDATE settings 
        SET company_name = ?, company_address = ?, company_phone = ?, company_email = ?, logo_base64 = ?, signature_text = ?, updated_at = CURRENT_TIMESTAMP
        WHERE id = 1
        "#
    )
    .bind(&setting.company_name)
    .bind(&setting.company_address)
    .bind(&setting.company_phone)
    .bind(&setting.company_email)
    .bind(&setting.logo_base64)
    .bind(&setting.signature_text)
    .execute(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}
