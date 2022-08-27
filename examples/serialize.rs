use geoip::model;

fn main() {
    let mut geodata = model::Geodata::default();
    println!("{:?}", geodata);

    let parsed = serde_json::to_string(&geodata).unwrap();
    println!("{}", parsed);
}