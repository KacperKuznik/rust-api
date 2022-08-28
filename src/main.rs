#[macro_use] extern crate rocket;
use rocket::serde::{Serialize, json::Json};
use rand::Rng;

#[serde(crate = "rocket::serde")]

#[derive(Serialize)]
struct FuelUsage {
    fuelUsage: f32
}

#[get("/?<distance>&<yearOfProduction>&<fuelUsagePer100KM>")]
fn calculate_diesel_fuel_usage(distance: u32, yearOfProduction: u16,  fuelUsagePer100KM: u16) -> Json<FuelUsage> {
    let distance = distance as f32;
    let fuelUsagePer100KM = fuelUsagePer100KM as f32;
    let fuelUsage = fuelUsagePer100KM/100.0*distance;
    Json(FuelUsage {fuelUsage: fuelUsage})
}

#[derive(Serialize)]
struct FailProbability {
    failProbability: f32
}

#[get("/?<vin>")]
fn calculate_probability(vin: &str) -> Json<FailProbability> {
    assert!(vin != "");
    let mut rng = rand::thread_rng();
    let probability = rng.gen_range(0..101) as  f32;
    Json(FailProbability {failProbability: probability/100.0})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/calculateDisselUsageForDistance", routes![calculate_diesel_fuel_usage])
        .mount("/probabilityOfUnitInjectorFail", routes![calculate_probability])
}