use rocket::*;

mod business;
use business::*;
use rocket::http::Status;
use rocket::serde::json::Json;

mod cors;
use cors::Cors;

mod test;


#[get("/hello/<name>")]
fn hello(name: String) -> String {
   format!("Hello {}!", name)
}

#[get("/businesses")]
fn get_businesses() -> Json<Vec<Business>> {
    let businesses = load_businesses();
    Json(businesses)
}

#[post("/business", format="json", data="<business>")]
fn create_business(business: Json<Business>) -> Status {
    let mut businesses = load_businesses();
    if let Some(_index) = businesses
        .iter()
        .position(|item| item.business_name == business.business_name)
    {
        return Status::Conflict;
    }
    businesses.push(business.0);
    save_businesses(&businesses);
    Status::Created
}

#[delete("/business", format = "json", data = "<business>")]
fn delete_business(business: Json<Business>) -> Status {
   let mut businesses = load_businesses();
   if let Some(index) = businesses
       .iter()
       .position(|item| item.business_name == business.business_name)
    {
       businesses.remove(index);
       save_businesses(&businesses);
       return Status::NoContent;
    } else {
       return Status::NotFound;
    } 
}

#[put("/business", format = "json", data="<business>")]
fn update_business(business: Json<Business>) -> Status {
    let mut businesses = load_businesses();
    if let Some(index) = businesses
       .iter()
       .position(|item| item.business_name == business.business_name)
    {
       businesses.remove(index);
       businesses.push(business.0);
       save_businesses(&businesses);
       return Status::Created;
    } else {
       return Status::NotFound;
    } 
}

#[options("/business")]
fn all_options() {
}

#[launch]
fn rocket() -> _ {
   rocket::build()
    .attach(Cors)
    .mount("/", routes![hello, get_businesses, create_business, delete_business, update_business, all_options])
}
