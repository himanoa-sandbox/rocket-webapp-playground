use diesel::RunQueryDsl;
use rocket_contrib::json::Json;

use diesel;
use helpers::mysql::DbConn;
use models::user::NewUser;
use schema::users::dsl::users;

#[post("/user", data = "<new_user>", format = "application/json")]
pub fn create_user(new_user: Json<NewUser>, db: DbConn) -> Json {
    let new_users = vec![NewUser {
        name: new_user.0.name,
    }];
    let result = diesel::insert_into(users).values(&new_users).execute(&*db);
    match result {
        Ok(_) => Json(json!({
            "status": "created"
        })),
        Err(_) => Json(json!({
            "status": "err"
        })),
    }
}
