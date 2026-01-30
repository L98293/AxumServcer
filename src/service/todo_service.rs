use sea_orm::DbErr;
use sea_orm::*;
use sea_orm::ActiveValue::Set;
use crate::entity::todo;
use crate::dto::request::create_to_do_list_request::CreateToDoListRequest;
use crate::dto::request::update_to_do_list_request::UpdateToDoListRequest;

pub struct ToDoService;

impl ToDoService {

    pub async fn create_todo_list (
        db: &DbConn,
        request: CreateToDoListRequest,
    ) -> Result<todo::Model, DbErr> {

        let new_todo_list = todo::ActiveModel {
        title: Set(request.title),
        content: Set(request.content),
        ..Default::default() // 지정하지 않은 값들은 기본값으로(예를 들어 id)
        };

        let result =
            todo::Entity::insert(new_todo_list)
                .exec_with_returning(db)
                .await?;

        Ok(result)
    }

    pub async fn read_todo_list (
        db: &DbConn,
        id: u64
    ) -> Result<todo::Model, DbErr> {

        let todo: Option<todo::Model> = todo::Entity::find_by_id(id)
            .one(db)
            .await?;

        match todo {
            Some(todo) => Ok(todo),
            None => Err(DbErr::Custom("Todo Not Found".to_string()))
        }
    }

    pub async fn read_all_todo_list (
        db: &DbConn
    ) -> Result<Vec<todo::Model>, DbErr> {

        let todo = todo::Entity::find()
            .all(db)
            .await?;

        Ok(todo)
    }

    pub async fn update_todo_list (
        db: &DbConn,
        id: u64,
        request: UpdateToDoListRequest,
    ) -> Result<todo::Model, DbErr> {

        let todo = todo::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Todo Not Found".to_string()))?;

        let mut todo_active: todo::ActiveModel = todo.into();
        todo_active.title = Set(request.title);
        todo_active.content = Set(request.content);

        let update_todo = todo_active.update(db).await?;

        Ok(update_todo)
    }

    pub async fn delete_todo_list (
        db: &DbConn,
        id: u64
    ) -> Result<(), DbErr> {

        todo::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Todo Not Found".to_string()))?;

        todo::Entity::delete_by_id(id)
            .exec(db)
            .await?;

        Ok(())
    }
}