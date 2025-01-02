use sea_orm::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "subscription")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = true)]
  pub id: i32,

  pub publisher_id: i32,

  pub user_id: i32,

  pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    belongs_to = "super::publisher::Entity",
    from = "Column::PublisherId",
    to = "super::publisher::Column::Id",
  )]
  Publisher,

  #[sea_orm(
    belongs_to = "super::user::Entity",
    from = "Column::UserId",
    to = "super::user::Column::Id",
  )]
  User,
}

impl Related<super::publisher::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Publisher.def()
  }
}

impl Related<super::user::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::User.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}
