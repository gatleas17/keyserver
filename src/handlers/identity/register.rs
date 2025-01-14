use {
    super::super::{validate_caip10_account, validate_identity_key, Response},
    crate::{
        auth::cacao::Cacao,
        error::{self},
        state::AppState,
    },
    axum::extract::{Json, State},
    serde::Deserialize,
    std::sync::Arc,
    validator::Validate,
};

#[derive(Deserialize)]
pub struct RegisterIdentityPayload {
    pub cacao: Cacao,
}

#[derive(Validate)]
pub struct RegisterIdentityParams {
    #[validate(custom = "validate_caip10_account")]
    account: String,
    #[validate(custom = "validate_identity_key")]
    identity_key: String,
    cacao: Cacao,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterIdentityPayload>,
) -> error::Result<Response> {
    let cacao = payload.cacao;
    cacao.verify()?;

    let identity_key = cacao.p.identity_key()?;
    let account = cacao.p.caip_10_address()?;

    let params = RegisterIdentityParams {
        account,
        identity_key,
        cacao,
    };

    params.validate()?;

    state
        .keys_persitent_storage
        .create_account_if_not_exists_and_add_identity_key(
            &params.account,
            &params.identity_key,
            &params.cacao,
        )
        .await?;

    Ok(Response::default())
}
