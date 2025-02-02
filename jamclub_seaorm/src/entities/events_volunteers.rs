//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "events_volunteers")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub event_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub volunteer_id: i32,
    pub volunteer_type: i32,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::events::Entity",
        from = "Column::EventId",
        to = "super::events::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Events,
    #[sea_orm(
        belongs_to = "super::volunteers::Entity",
        from = "Column::VolunteerId",
        to = "super::volunteers::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Volunteers,
}

impl Related<super::events::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Events.def()
    }
}

impl Related<super::volunteers::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Volunteers.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
