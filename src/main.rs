// 导入所需的依赖项
mod api; 
mod models;
mod repository;

use actix_web::{web::Data, App, HttpServer};
use api::user_api::{create_user, get_user, update_user, get_all_users};
use repository::mongodb_repo::MongoRepo;


// 使用宏在 actix 运行时内异步#[actix_web::main]运行函数。main该main函数执行以下操作：
// 使用结构体创建一个新服务器HttpServer，该结构体使用闭包来使用App实例来服务
// 传入请求。它还App注册hello处理程序。HttpServer是我们应用程序的支柱；
// 它负责请求处理、允许的最大连接数、分层安全性等，同时App处理应用程序逻辑，
// 如请求处理程序、中间件、路由等
// 将服务器配置为异步运行并处理 上的 HTTP 请求localhost:8080。

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 创建一个db变量以通过调用该方法建立与 MongoDB 的连接init()，
    // 并将其添加到该结构的新实例中Data，
    // 以便数据库状态可以在整个应用程序范围内可用。
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(create_user)
            .service(get_user)
            .service(update_user)
            .service(get_all_users)
    })
    .bind(("127.0.0.1", 4000))?
    .run()
    .await
}