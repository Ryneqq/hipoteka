use std::collections::BTreeMap;

use crate::{Koszt, KosztKoncowy};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Koszty {
    nazwa: String,
    okres: u64,
    koszty: Vec<Koszt>,
}

impl Koszty {
    pub fn oblicz(&self) -> BTreeMap<String, KosztKoncowy> {
        self.koszty.iter().map(|k| (k.nazwa(), k.oblicz(0.0, self.okres))).collect()
    }
}

pub fn koszty(path: &str) {
    let data = std::fs::read_to_string(path).unwrap();
    let koszty_mieszkania: Koszty = serde_json::from_str(&data).unwrap();
    let koszty = koszty_mieszkania.oblicz();
    let total: f64 = koszty.iter().map(|(_, v)| v.total()).sum();

    println!("Koszt utrzymania `{}` wynosi: {} zl", koszty_mieszkania.nazwa, total);

    for (_, koszt) in koszty.iter() {
        println!("    * {}", koszt);
    }

    println!("");
}