mod koszt;
mod kredyt;
mod koszty;
mod kalkulator;
mod mapa_rat;
mod nadplaty;

use kredyt::*;
use koszty::*;
use mapa_rat::*;
use kalkulator::*;

pub use koszt::*;
pub use nadplaty::*;

fn main() {
    // oferta("data/kredyt/mbank.json");
    // oferta("data/kredyt/mbank_nadplata_new.json");
    // oferta("data/kredyt/santander.json");
    oferta("data/kredyt/santander_25.json");
    // oferta("data/kredyt/santander_nadplata_new.json");
    // oferta("data/kredyt/millenium.json");
    // oferta("data/kredyt/pekao_sa_ubezpiecznie.json");
    // oferta("data/kredyt/pekao_sa.json");
    // oferta("data/kredyt/pekao_sa_25.json");
    oferta("data/kredyt/pekao_sa_25_copy.json");
    // oferta("data/kredyt/alior_bank.json");
    // oferta("data/kredyt/alior_bank_nadplata_new.json");
    // oferta("data/kredyt/alior_bank_25.json");
    // koszty("data/koszty/pawia.json");
    // koszty("data/koszty/uznanskiego.json");
}