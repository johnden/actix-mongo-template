// 导入所需的依赖项
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

// 使用derive宏生成对格式化输出、序列化和反序列化数据结构的实现支持。
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    // 创建 User具有所需属性的结构。我们还向id属性添加了字段属性，以重命名并忽略该字段（如果该字段为空）。
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub location: String,
    pub title: String,
}
