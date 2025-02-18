// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use crate::core::types::{
    ApplicationId, BaseId, EventChannelSet, EventTypeName, MessageId, MessageIdOrUid, MessageUid,
    OrganizationId,
};
use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::{entity::prelude::*, Condition};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "message")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: MessageId,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub org_id: OrganizationId,
    pub app_id: ApplicationId,
    pub event_type: EventTypeName,
    pub uid: Option<MessageUid>,
    #[sea_orm(column_type = "JsonBinary")]
    pub payload: Json,
    pub channels: Option<EventChannelSet>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::application::Entity",
        from = "Column::AppId",
        to = "super::application::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Application,
    #[sea_orm(has_many = "super::messagedestination::Entity")]
    Messagedestination,
}

impl Related<super::application::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Application.def()
    }
}

impl Related<super::messagedestination::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Messagedestination.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(MessageId::new(None, None)),
            created_at: Set(Utc::now().into()),
            updated_at: Set(Utc::now().into()),
            ..ActiveModelTrait::default()
        }
    }

    fn before_save(mut self, _insert: bool) -> Result<Self, DbErr> {
        self.updated_at = Set(Utc::now().into());
        Ok(self)
    }
}

impl Entity {
    pub fn secure_find(app_id: ApplicationId) -> Select<Entity> {
        Self::find().filter(Column::AppId.eq(app_id))
    }

    pub fn secure_find_by_id(app_id: ApplicationId, id: MessageId) -> Select<Entity> {
        Self::secure_find(app_id).filter(Column::Id.eq(id))
    }

    pub fn secure_find_by_id_or_uid(
        app_id: ApplicationId,
        id_or_uid: MessageIdOrUid,
    ) -> Select<Entity> {
        Self::secure_find(app_id).filter(
            Condition::any()
                .add(Column::Id.eq(id_or_uid.to_owned()))
                .add(Column::Uid.eq(id_or_uid)),
        )
    }
}
