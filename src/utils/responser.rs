use actix_http::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, HttpResponseBuilder, Responder};
use serde::Serialize;
use serde_json::Number;

#[derive(Serialize, Debug)]
pub struct BodyBuilder {
    message: String,
    #[serde(skip_serializing)]
    status_code: StatusCode,
}

#[derive(Serialize, Debug)]
pub struct DataResponseBuilder<T>
where
    T: Serialize,
{
    message: String,
    data: T,
    #[serde(skip_serializing)]
    status_code: StatusCode,
}

impl From<BodyBuilder> for HttpResponse {
    fn from(value: BodyBuilder) -> Self {
        HttpResponseBuilder::new(value.status_code).json(value)
    }
}

impl<T> From<DataResponseBuilder<T>> for HttpResponse
where
    T: Serialize,
{
    fn from(value: DataResponseBuilder<T>) -> Self {
        HttpResponseBuilder::new(value.status_code).json(value)
    }
}

impl BodyBuilder {
    pub fn status(mut self, status_code: StatusCode) -> Self {
        self.status_code = status_code;
        self
    }

    pub fn add_data<T>(self, data: T) -> DataResponseBuilder<T>
    where
        T: Serialize,
    {
        DataResponseBuilder {
            message: self.message,
            data,
            status_code: self.status_code,
        }
    }

    pub fn build_with_cookie(self, cookie: actix_web::cookie::Cookie<'_>) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code)
            .cookie(cookie)
            .json(self)
    }
}


impl From<String> for BodyBuilder {
    fn from(message: String) -> BodyBuilder {
        BodyBuilder {
            message,
            status_code: StatusCode::OK,
        }
    }
}

 impl From<&str> for BodyBuilder {
     fn from(message: &str) -> BodyBuilder {
         BodyBuilder::from(String::from(message))
     }
 }



impl Responder for BodyBuilder {
    type Body = BoxBody;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponseBuilder::new(self.status_code).json(self)
    }
}

impl<T> DataResponseBuilder<T>
where
    T: Serialize,
{
    pub fn status(mut self, status_code: StatusCode) -> Self {
        self.status_code = status_code;
        self
    }

    pub fn build_with_cookie(self, cookie: actix_web::cookie::Cookie<'_>) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code)
            .cookie(cookie)
            .json(self)
    }

}

impl<T> Responder for DataResponseBuilder<T>
where
    T: Serialize,
{
    type Body = BoxBody;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponseBuilder::new(self.status_code).json(self)
    }
}
