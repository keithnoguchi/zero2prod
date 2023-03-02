use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

pub(crate) async fn subscribe(
    form: web::Form<FormData>,
    db_pool: web::Data<PgPool>,
) -> HttpResponse {
    info!(
        "Adding '{}' '{}' as a new subscriber",
        form.email, form.name,
    );
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now(),
    )
    .execute(db_pool.get_ref())
    .await
    {
        Ok(_) => {
            info!("new subscriber details have been saved");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            error!("failed to execute query: {e:?}");
            HttpResponse::InternalServerError().finish()
        }
    }
}
