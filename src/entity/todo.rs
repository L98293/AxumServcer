use serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "ToDoList")]
pub struct Model {

    #[sea_orm(primary_key)]
    pub id: u64,
    pub title: String,
    pub content: String
}

// 외래 키 정의 용
// 외래 키가 없는 경우에도 SeaORM을 사용하는 경우 Relation을 필수로 존재해야함
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

// 얘도 필수로 구현되어 있어야함
// 아래처럼 비어 있으면 기본 설정 사용
impl ActiveModelBehavior for ActiveModel {
}