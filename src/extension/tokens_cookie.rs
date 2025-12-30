use actix_web::cookie::{Cookie, SameSite};

pub struct TokensCookie<'a> {
    pub refresh_token: Cookie<'a>,
}

impl<'a> TokensCookie<'a> {
    pub fn from_tokens(value: Option<&str>, lax_domain: &'a str) -> Self {
            match value {
                Some(tokens) => TokensCookie { refresh_token: convert_refresh_token_to_cookie(tokens.to_string(), lax_domain),
                },
                None => {
                    let refresh_token = Cookie::build("refresh_token", "")
                        .path("/")
                        .domain(lax_domain)
                        .http_only(true)
                        .same_site(SameSite::Lax)
                        .max_age(time::Duration::seconds(0))
                        .finish();
                    TokensCookie { refresh_token }
                }
            }
        }
}

pub fn convert_refresh_token_to_cookie(refresh_token: String, lax_domain: &'_ str) -> Cookie<'_> {
    Cookie::build("refresh_token", refresh_token)
        .path("/")
        .http_only(true)
        .secure(true)
        .domain(lax_domain)
        .same_site(SameSite::Lax)
        .finish()
}
