use govuk_prototype_rs;
use rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    govuk_prototype_rs::rocket().launch().await?;
    Ok(())
}
