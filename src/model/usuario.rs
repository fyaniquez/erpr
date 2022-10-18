use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

/// mod_datos::usuario
/// autor: fyaniquez
/// fecha: 03-03-2022

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Usuario {
    pub id: i64,
    pub nombre: String,
    pub documento: String,
    pub email: String,
    pub password_digest: String,
    pub activo: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
/*
/// recupera un registro a partir del email
pub async fn fila_x_email(email: &str, req: &Request<State>) -> Result<Usuario> {
    const SELECT: &str = "SELECT {} FROM usuarios WHERE email = $1";

    let db_pool: PgPool = req.state().db_pool.clone();
    let select = SELECT.replacen("{}", COLUMNAS, 1);
    let usuario: Usuario = query_as(&select)
        .bind(email.clone())
        .fetch_one(&db_pool)
        .await
        .map_err(|e| Error::from_str(409, e))?;
    Ok(usuario)
}
*/
