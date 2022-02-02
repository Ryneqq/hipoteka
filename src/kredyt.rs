use std::collections::BTreeMap;

use serde::{Serialize, Deserialize};

use crate::{ Koszt, Operator, Okres, KosztKoncowy, Nadplaty };

#[derive(Serialize, Deserialize)]
pub struct Kredyt {
    #[serde(rename = "nazwa")]
    pub nazwa: String,

    #[serde(rename = "wartosc_hipoteki")]
    pub wartosc_hipoteki: f64,

    #[serde(rename = "wklad_wlasny")]
    pub wklad_wlasny: f64,

    #[serde(rename = "okres_kredytowania")]
    pub okres_kredytowania: u64,

    #[serde(rename = "oprocentowanie")]
    pub oprocentowanie: f64,

    #[serde(rename = "koszty")]
    pub koszty: Vec<Koszt>,

    #[serde(rename = "nadplaty")]
    pub nadplaty: Nadplaty,

    #[serde(rename = "splata")]
    pub splata: Splata,
}

impl Kredyt {
    pub fn koszt_nieruchomosci(&self) -> BTreeMap<String, KosztKoncowy> {
        let mut koszty: BTreeMap<String, KosztKoncowy> = self.koszty.iter().map(|k| (k.nazwa(), k.oblicz(self.wartosc_hipoteki - self.wartosc_hipoteki * self.wklad_wlasny / 100.0, self.okres_kredytowania))).collect();
        let pcc = self.wartosc_hipoteki * 2.0 / 100.0;
        let pcc = KosztKoncowy::new(
                Koszt::builder()
                    .nazwa("PCC")
                    .wartosc(2.0)
                    .operator(Operator::Procent)
                    .okres(Okres::Jednorazowy)
                    .build(),
                pcc
            );

        let rata = self.rata_rowna();
        let raty = rata * self.okres_kredytowania as f64;
        let raty = KosztKoncowy::new(
            Koszt::builder()
                .nazwa("Raty")
                .wartosc(rata)
                .operator(Operator::Stala)
                .okres(Okres::Miesieczny)
                .build(),
            raty
        );

        koszty.insert("PCC".to_string(), pcc);
        koszty.insert("Raty".to_string(), raty);

        koszty
    }

    pub fn rata_rowna(&self) -> f64 {
        let kwota_kredytu = self.wartosc_hipoteki - self.wartosc_hipoteki * self.wklad_wlasny / 100.0;
        let liczba_rat_rocznie = 12.0;
        let oprocentowanie = self.oprocentowanie / 100.0;

        kwota_kredytu * oprocentowanie / (liczba_rat_rocznie * (1.0 - (liczba_rat_rocznie / (liczba_rat_rocznie + oprocentowanie)).powf(self.okres_kredytowania as f64)))
    }
}

#[derive(Serialize, Deserialize)]
pub struct Rata {
    #[serde(rename = "kapital")]
    kapital: f64,

    #[serde(rename = "odsetki")]
    odsetki: f64,

    #[serde(rename = "delta_kapital")]
    delta_kapital: f64,

    #[serde(rename = "delta_odsetki")]
    delta_odsetki: f64,

    #[serde(rename = "operator")]
    operator: Operator,

    #[serde(rename = "okres")]
    okres: Okres,

    #[serde(rename = "nadplata")]
    nadplata: Option<f64>,
}

impl Rata {
    pub fn oblicz(&self, okres_kredytowania: u64) -> f64 {
        // TODO
        // let nadplata_w_stosunku_okresu = self.nadplata / okres_kredytowania as f64;

        (0..okres_kredytowania).map(|numer_raty| {
            let kapital = self.kapital + self.delta_kapital * numer_raty as f64;
            let odsetki = self.odsetki + self.delta_odsetki * numer_raty as f64;

            debug_assert!(odsetki >= 0.0);

            kapital + odsetki
        })
        .sum()
    }

    pub fn wartosc(&self) -> f64 {
        self.kapital + self.odsetki + self.nadplata.unwrap_or_default()
    }
}


#[derive(Serialize, Deserialize)]
pub struct Splata {
    #[serde(flatten)]
    wartosc: Koszt
}

// pub fn oferta(path: &str) {
//     let data = std::fs::read_to_string(path).unwrap();
//     let kredyt: Kredyt = serde_json::from_str(&data).unwrap();
//     let koszty = kredyt.koszt_nieruchomosci();
//     let total: f64 = koszty.iter().map(|(_, v)| v.total()).sum();
//     let koszt_kredytu = total - kredyt.wartosc_hipoteki;

//     println!("Koszt nieruchomosci w banku `{}` wynosi: {} zl", kredyt.nazwa, total);

//     for (_, koszt) in koszty.iter() {
//         println!("    * {}", koszt);
//     }

//     let miesieczne_okresowe: f64 = koszty.iter()
//         .filter(|(_, v)| v.okres() == Okres::Miesieczny)
//         .map(|(_, v)| v.wartosc())
//         .sum();

//     let miesieczne_stale: f64 = koszty.iter()
//         .filter(|(_, v)| v.okres() == Okres::Miesieczny && v.okresow().is_none())
//         .map(|(_, v)| v.wartosc())
//         .sum();

//     println!("Maksymalne miesięczne zobowiązanie: {}zl", miesieczne_okresowe);
//     println!("Standardowe miesięczne zobowiązanie: {}zl", miesieczne_stale);
//     println!("Koszt kredytu: {}zl", koszt_kredytu);
//     println!("");
// }
