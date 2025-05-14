use chrono::{TimeDelta, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};

use crate::structures::{
    auth_structures::login_structures::Claims, general::AccountInfo, static_vars::JWT_SECRET,
};

pub fn generate_token(user: AccountInfo) -> Result<String, jsonwebtoken::errors::Error> {
    let exp = usize::try_from((Utc::now() + TimeDelta::try_days(9_999_999).unwrap()).timestamp())
        .unwrap();

    let claims = Claims {
        exp,
        id: user.id,
        phone_number: user.phone_number,
        username: user.username,
        is_driver: user.is_driver,
        is_owner: user.is_owner,
        address: user.address,
        id_approved: user.id_approved,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
}
