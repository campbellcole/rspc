use rspc::{Config, Error, ErrorCode, Router};
use serde_json::json;

pub enum MyCustomError {
    IAmBroke,
}

impl Into<Error> for MyCustomError {
    fn into(self) -> Error {
        match self {
            MyCustomError::IAmBroke => {
                Error::new(ErrorCode::InternalServerError, "I am broke".into())
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let router = <Router>::new()
        .config(Config::new().export_ts_bindings("./ts"))
        .query("ok", |_, _args: ()| {
            Ok("Hello World".into()) as Result<String, Error>
        })
        .query("err", |_, _args: ()| {
            Err(Error::new(
                ErrorCode::BadRequest,
                "This is a custom error!".into(),
            )) as Result<String, Error>
        })
        .query("customErr", |_, _args: ()| {
            Err(MyCustomError::IAmBroke) as Result<String, MyCustomError>
        })
        .build();

    println!(
        "{:#?}",
        router.exec_query((), "ok", json!(null)).await.unwrap()
    );
    println!("{:#?}", router.exec_query((), "err", json!(null)).await);
    println!(
        "{:#?}",
        router.exec_query((), "customErr", json!(null)).await
    );
}
