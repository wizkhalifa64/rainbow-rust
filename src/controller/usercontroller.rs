use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::{json, Value};

use crate::{
    middleware::validator::ValidatedJson,
    models::{
        user::{LoginUserSchema, RegisterUserSchema, TokenClaims, User},
        userresponse::FilteredUser,
    },
    AppState,
};
use bcrypt::{hash, verify};
pub async fn register_handler(
    State(data): State<Arc<AppState>>,
    ValidatedJson(body): ValidatedJson<RegisterUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let user_exist: Option<bool> =
        sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM tbl_users WHERE email = $1)")
            .bind(body.email.to_owned().to_ascii_lowercase())
            .fetch_one(&data.db)
            .await
            .map_err(|e| {
                let error_response = json!({
                    "status": "fail",
                    "message": format!("{}", e),
                });
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
            })?;

    if let Some(exists) = user_exist {
        if exists {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": "User with that email already exists",
            });
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }
    }
    let hashed = hash(body.password.as_bytes(), 5).expect("Error");
    sqlx::query_as!(
        User,
        "INSERT INTO
    tbl_users (
        name,
        email,
        password,
        role,
        address,
        area_list,
        qualification,
        employee_id
    )
VALUES
    ($1,$2,$3,$4,$5,$6,$7,$8) RETURNING *",
        body.name.to_string(),
        body.email.to_string().to_ascii_lowercase(),
        hashed,
        body.role,
        body.address,
        body.area_list,
        body.qualification,
        body.employee_id
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Database error: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;
    let success_response = serde_json::json!({
        "status": "success",
        "message": "User created successfully"
    });

    Ok((StatusCode::CREATED, Json(success_response)))
}

pub async fn login_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<LoginUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM tbl_users WHERE email = $1",
        body.email.to_ascii_lowercase()
    )
    .fetch_optional(&data.db)
    .await
    .map_err(|e| {
        let error_response = json!({
            "status": "error",
            "message": format!("Database error: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?
    .ok_or_else(|| {
        let error_response = json!({
            "status": "fail",
            "message": "Invalid email",
        });
        (StatusCode::BAD_REQUEST, Json(error_response))
    })?;
    let compare_pass = match verify(body.password.as_bytes(), &user.password) {
        Ok(valid) => valid,
        Err(_) => false,
    };
    if !compare_pass {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Invalid Credential"
        });
        return Err((StatusCode::BAD_REQUEST, Json(error_response)));
    }
    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = 3600;
    let claims: TokenClaims = TokenClaims {
        sub: user.id.to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data.env.jwt_secret.as_ref()),
    )
    .unwrap();
    let user_details = FilteredUser {
        id: user.id,
        email: user.email.to_owned(),
        name: user.name.to_owned(),
        photo: user.photo.to_owned(),
        role: user.role,
        verified: user.verified,
        createdAt: user.created_at.unwrap(),
        updatedAt: user.updated_at.unwrap(),
        areaList: user.area_list,
    };
    let user_response = serde_json::json!({"status": "success","data": serde_json::json!({
        "user": user_details,
        "token":token
    })});
    Ok(Json(user_response))
}

// pub async fn create_agent(State(data): State<Arc<AppState>>) {
//     let agent_exist: Option<bool> =
//         sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM users WHERE email = $1)")
//             .bind(body.email.to_owned().to_ascii_lowercase())
//             .fetch_one(&data.db)
//             .await
//             .map_err(|e| {
//                 let error_response = json!({
//                     "status": "fail",
//                     "message": format!("Database error: {}", e),
//                 });
//                 (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
//             })?;

//     if let Some(exists) = agent_exist {
//         if exists {
//             let error_response = serde_json::json!({
//                 "status": "fail",
//                 "message": "User with that email already exists",
//             });
//             return Err((StatusCode::CONFLICT, Json(error_response)));
//         }
//     }
//     let hashed = hash(body.password.as_bytes(), 5).expect("Error");
//     let user = sqlx::query_as!(
//         User,
//         "INSERT INTO users (name,email,password) VALUES ($1, $2, $3) RETURNING *",
//         body.name.to_string(),
//         body.email.to_string().to_ascii_lowercase(),
//         hashed,
//     )
//     .fetch_one(&data.db)
//     .await
//     .map_err(|e| {
//         let error_response = serde_json::json!({
//             "status": "fail",
//             "message": format!("Database error: {}", e),
//         });
//         (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
//     })?;

//     let now = chrono::Utc::now();
//     let iat = now.timestamp() as usize;
//     let exp = 3600;
//     let claims: TokenClaims = TokenClaims {
//         sub: user.id.to_string(),
//         exp,
//         iat,
//     };

//     let token = encode(
//         &Header::default(),
//         &claims,
//         &EncodingKey::from_secret(data.env.jwt_secret.as_ref()),
//     )
//     .unwrap();

//     let user_response = serde_json::json!({"status": "success","data": serde_json::json!({
//         "user": filter_user_record(&user),
//         "token":token
//     })});
//     Ok(Json(user_response))
// }
