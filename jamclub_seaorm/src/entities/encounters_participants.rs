//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use super::sea_orm_active_enums::ParticipantType;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "encounters_participants")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub encounter_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub participant_id: i32,
    pub participant_type: ParticipantType,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::encounters::Entity",
        from = "Column::EncounterId",
        to = "super::encounters::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Encounters,
    #[sea_orm(
        belongs_to = "super::members::Entity",
        from = "Column::ParticipantId",
        to = "super::members::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Members,
}

impl Related<super::encounters::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Encounters.def()
    }
}

impl Related<super::members::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Members.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
