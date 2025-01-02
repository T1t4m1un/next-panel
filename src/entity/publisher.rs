use sea_orm::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "publisher")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = true)]
  pub id: i32,

  #[sea_orm(unique)]
  pub name: String,

  pub description: String,

  pub token: String,

  pub created_at: DateTimeUtc,
}


#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(has_many = "super::subscription::Entity")]
  Subscription,
}

impl ActiveModelBehavior for ActiveModel {}
