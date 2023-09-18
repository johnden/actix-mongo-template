// 导入所需的依赖项
use crate::{models::user_model::User, repository::mongodb_repo::MongoRepo};
use actix_web::{
    post, get, put, delete,
    web::{Data, Json, Path},
    HttpResponse,
};
use mongodb::bson::oid::ObjectId;

// 使用路由宏指定HTTP方法和对应的路由
// 创建一个create_user处理程序，该处理程序接受db、 
// 的类型MongoRepo和 anew_user作为参数。在处理程序内部，
// 我们创建了一个用于data创建用户的变量，使用该方法将其插入数据库db.create_user，
// 如果插入成功或有错误，则返回正确的响应。
#[post("/user")]
pub async fn create_user(db: Data<MongoRepo>, new_user: Json<User>) -> HttpResponse {
    let data = User {
        id: None,
        name: new_user.name.to_owned(),
        location: new_user.location.to_owned(),
        title: new_user.title.to_owned(),
    };
    let user_detail = db.create_user(data).await;
    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// 创建一个get_user处理程序，该处理程序接受db、 
// 的类型MongoRepo和用于访问路由路径的 apath作为参数。
// 在处理程序内部，我们创建了一个id变量来获取用户的id，
// 使用该方法从数据库中获取用户的详细信息db.get_user。如果请求成功，
// 我们将返回正确的响应；如果有错误，我们将返回正确的响应。
#[get("/user/{id}")]
pub async fn get_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let user_detail = db.get_user(&id).await;
    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// 创建一个处理程序，该处理程序update_user接受的类型作为参数。
// 在处理程序内部，我们创建了一个变量来获取用户的id，
// 并使用该方法通过传入更新的用户信息来更新数据库中的用户详细信息。
// 最后，我们检查更新是否成功，并返回更新后的用户或错误（如果有）。
// dbMongoRepopathnew_useriddb.update_user
#[put("/user/{id}")]
pub async fn update_user(
    db: Data<MongoRepo>,
    path: Path<String>,
    new_user: Json<User>,
) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    let data = User {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        name: new_user.name.to_owned(),
        location: new_user.location.to_owned(),
        title: new_user.title.to_owned(),
    };
    let update_result = db.update_user(&id, data).await;
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_user_info = db.get_user(&id).await;
                return match updated_user_info {
                    Ok(user) => HttpResponse::Ok().json(user),
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                };
            } else {
                return HttpResponse::NotFound().body("No user found with specified ID");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/user/{id}")]
pub async fn delete_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    let result = db.delete_user(&id).await;
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return HttpResponse::Ok().json("User successfully deleted!");
            } else {
                return HttpResponse::NotFound().json("User with specified ID not found!");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/users")]
pub async fn get_all_users(db: Data<MongoRepo>) -> HttpResponse {
    let users = db.get_all_users().await;
    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}