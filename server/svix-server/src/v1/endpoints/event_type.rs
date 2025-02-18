// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use crate::{
    core::types::EventTypeName,
    error::{HttpError, Result},
    v1::utils::{
        api_not_implemented, EmptyResponse, ListResponse, ModelIn, ModelOut, ValidatedJson,
        ValidatedQuery,
    },
};
use axum::{
    extract::{Extension, Path},
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use hyper::StatusCode;
use sea_orm::{entity::prelude::*, ActiveValue::Set, QueryOrder};
use sea_orm::{ActiveModelTrait, DatabaseConnection, QuerySelect};
use serde::{Deserialize, Serialize};
use svix_server_derive::{ModelIn, ModelOut};
use validator::Validate;

use crate::core::security::Permissions;
use crate::db::models::eventtype;
use crate::v1::utils::Pagination;

#[derive(Clone, Debug, PartialEq, Deserialize, Validate, ModelIn)]
#[serde(rename_all = "camelCase")]
struct EventTypeIn {
    name: EventTypeName,
    description: String,
    #[serde(default, rename = "archived")]
    deleted: bool,
    schemas: Option<serde_json::Value>,
}

// FIXME: This can and should be a derive macro
impl ModelIn for EventTypeIn {
    type ActiveModel = eventtype::ActiveModel;

    fn update_model(self, model: &mut Self::ActiveModel) {
        model.name = Set(self.name);
        model.description = Set(self.description);
        model.deleted = Set(self.deleted);
        model.schemas = Set(self.schemas);
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Validate, ModelIn)]
#[serde(rename_all = "camelCase")]
struct EventTypeUpdate {
    description: String,
    #[serde(default, rename = "archived")]
    deleted: bool,
    schemas: Option<serde_json::Value>,
}

// FIXME: This can and should be a derive macro
impl ModelIn for EventTypeUpdate {
    type ActiveModel = eventtype::ActiveModel;

    fn update_model(self, model: &mut Self::ActiveModel) {
        model.description = Set(self.description);
        model.deleted = Set(self.deleted);
        model.schemas = Set(self.schemas);
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, ModelOut)]
#[serde(rename_all = "camelCase")]
struct EventTypeOut {
    name: EventTypeName,
    description: String,
    #[serde(rename = "archived")]
    deleted: bool,
    schemas: Option<serde_json::Value>,

    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

// FIXME: This can and should be a derive macro
impl From<eventtype::Model> for EventTypeOut {
    fn from(model: eventtype::Model) -> Self {
        Self {
            name: model.name,
            description: model.description,
            deleted: model.deleted,
            schemas: model.schemas,

            created_at: model.created_at.into(),
            updated_at: model.updated_at.into(),
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct ListFetchOptions {
    #[serde(default)]
    pub include_archived: bool,
}

async fn list_event_types(
    Extension(ref db): Extension<DatabaseConnection>,
    pagination: ValidatedQuery<Pagination<EventTypeName>>,
    fetch_options: ValidatedQuery<ListFetchOptions>,
    permissions: Permissions,
) -> Result<Json<ListResponse<EventTypeOut>>> {
    let limit = pagination.limit;
    let iterator = pagination.iterator.clone();

    let mut query = eventtype::Entity::secure_find(permissions.org_id)
        .order_by_asc(eventtype::Column::Name)
        .limit(limit + 1);

    if !fetch_options.include_archived {
        query = query.filter(eventtype::Column::Deleted.eq(false));
    }

    if let Some(iterator) = iterator {
        query = query.filter(eventtype::Column::Name.gt(iterator));
    }

    Ok(Json(EventTypeOut::list_response(
        query.all(db).await?.into_iter().map(|x| x.into()).collect(),
        limit as usize,
    )))
}

async fn create_event_type(
    Extension(ref db): Extension<DatabaseConnection>,
    ValidatedJson(data): ValidatedJson<EventTypeIn>,
    permissions: Permissions,
) -> Result<(StatusCode, Json<EventTypeOut>)> {
    let evtype =
        eventtype::Entity::secure_find_by_name(permissions.org_id.clone(), data.name.to_owned())
            .one(db)
            .await?;
    let ret = match evtype {
        Some(evtype) => {
            if evtype.deleted {
                let mut evtype: eventtype::ActiveModel = evtype.into();
                evtype.deleted = Set(false);
                data.update_model(&mut evtype);
                evtype.update(db).await?
            } else {
                return Err(HttpError::conflict(
                    Some("event_type_exists".to_owned()),
                    Some("An event_type with this name already exists".to_owned()),
                )
                .into());
            }
        }
        None => {
            let evtype = eventtype::ActiveModel {
                org_id: Set(permissions.org_id.clone()),
                ..data.into()
            };
            evtype.insert(db).await?
        }
    };
    Ok((StatusCode::CREATED, Json(ret.into())))
}

async fn get_event_type(
    Extension(ref db): Extension<DatabaseConnection>,
    Path(evtype_name): Path<EventTypeName>,
    permissions: Permissions,
) -> Result<Json<EventTypeOut>> {
    let evtype = eventtype::Entity::secure_find_by_name(permissions.org_id, evtype_name)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;
    Ok(Json(evtype.into()))
}

async fn update_event_type(
    Extension(ref db): Extension<DatabaseConnection>,
    Path(evtype_name): Path<EventTypeName>,
    ValidatedJson(data): ValidatedJson<EventTypeUpdate>,
    permissions: Permissions,
) -> Result<Json<EventTypeOut>> {
    let evtype = eventtype::Entity::secure_find_by_name(permissions.org_id.clone(), evtype_name)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;

    let mut evtype: eventtype::ActiveModel = evtype.into();
    data.update_model(&mut evtype);

    let ret = evtype.update(db).await?;
    Ok(Json(ret.into()))
}

async fn delete_event_type(
    Extension(ref db): Extension<DatabaseConnection>,
    Path(evtype_name): Path<EventTypeName>,
    permissions: Permissions,
) -> Result<(StatusCode, Json<EmptyResponse>)> {
    let evtype = eventtype::Entity::secure_find_by_name(permissions.org_id, evtype_name)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;

    let mut evtype: eventtype::ActiveModel = evtype.into();
    evtype.deleted = Set(true);
    evtype.update(db).await?;
    Ok((StatusCode::NO_CONTENT, Json(EmptyResponse {})))
}

pub fn router() -> Router {
    Router::new()
        .route("/event-type/", get(list_event_types))
        .route("/event-type/", post(create_event_type))
        .route(
            "/event-type/:event_type_name/",
            get(get_event_type)
                .put(update_event_type)
                .delete(delete_event_type),
        )
        .route(
            "/event-type/schema/generate-example/",
            post(api_not_implemented),
        )
}
