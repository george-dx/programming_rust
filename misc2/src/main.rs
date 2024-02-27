struct City {
    name: String,
    population: i64,
    country: String,
}

fn city_population_descending(city: &City) -> i64 {
    -city.population
}

fn sort_cities(cities: &mut Vec<City>) {
    cities.sort_by_key(city_population_descending);
}

fn sort_cities_with_closure(cities: &mut Vec<City>) {
    cities.sort_by_key(|city| -city.population);
}

fn main() {
    println!("Hello, world!");
}
