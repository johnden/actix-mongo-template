// 导入所需的依赖项
use std::env;
extern crate dotenv;
use dotenv::dotenv;

use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};
use crate::models::user_model::User;
// 创建一个MongoRepo带有字段的结构体col来访问 MongoDB 集合
pub struct MongoRepo {
    col: Collection<User>,
}

// MongoRepo创建一个向结构体添加方法的实现块
impl MongoRepo {
    // init向实现块添加一个方法来加载环境变量，创建到数据库的连接，并返回MongoRepo结构体的实例
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri)
            .await
            .expect("error connecting to database");
        let db = client.database("rustDB");
        let col: Collection<User> = db.collection("User");
        MongoRepo { col }
    }

    // 添加一个create_user方法，该方法接受self和new_user作为参数
    // 并返回创建的用户或错误。在该方法内部，我们使用Userstruct
    // 创建了一个新文档。然后我们使用self引用MongoRepo结构体来访问
    // insert_one集合中的函数来创建新用户并处理错误。最后我们返回了创建的用户信息
    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            name: new_user.name,
            location: new_user.location,
            title: new_user.title,
        };
        let user = self
            .col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating user");
        Ok(user)
    }

    // 添加一个get_user方法，该方法接受self和id作为参数并返回用户详细信息或错误。
    // 在方法内部，我们将 转换id为 anObjectId 并将其用作过滤器来获取匹配的文档。
    // 然后，我们使用self引用该MongoRepo结构体来访问find_one集合中的函数，
    // 以获取用户的详细信息并处理错误。最后我们返回了创建的用户信息。
    pub async fn get_user(&self, id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting user's detail");
        Ok(user_detail.unwrap())
    }

    // 添加一个update_user方法，该方法接受 self、id和new_user参数
    // 并返回更新的用户详细信息或错误。在方法内部，我们将 转换id为ObjectId，
    // 创建一个filter 变量来获取我们想要更新的匹配文档，并使用doc宏来更新文档字段。
    // 然后，我们使用self引用该MongoRepo结构体来访问update_one集合中的函数，
    // 以更新filter与指定匹配的用户并处理错误。最后我们返回了更新后的用户信息。
    pub async fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_user.id,
                    "name": new_user.name,
                    "location": new_user.location,
                    "title": new_user.title
                },
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating user");
        Ok(updated_doc)
    }

    // 添加一个delete_user方法，该方法接受 self和id作为参数并返回已删除的用户详细信息或错误。
    // 在该方法内部，我们将 转换id为 anObjectId并创建一个filter 
    // 变量来获取我们要删除的匹配文档。然后，我们使用self引用该MongoRepo结构体
    // 来访问delete_one集合中的函数，以删除filter与指定匹配的用户并处理错误。
    // 最后我们返回了删除的用户信息。
    pub async fn delete_user(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting user");
        Ok(user_detail)
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let mut cursors = self
            .col
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of users");
        let mut users: Vec<User> = Vec::new();
        while let Some(user) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            users.push(user)
        }
        Ok(users)
        }
}
