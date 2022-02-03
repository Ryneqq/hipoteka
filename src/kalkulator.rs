use std::{collections::BTreeMap, fmt, cmp};

use crate::{KosztKoncowy, kredyt::Kredyt, Koszt, Operator, Okres, Nadplaty, mapa_rat::{Rata, KalkulatorRaty, MapaRat}};

// TODO % posiadanej hipoteki po x latach
// TODO nadplaty

pub struct Kalkulator {
    nazwa: String,
    wartosc_hipoteki: f64,
    wklad_wlasny: f64,
    oprocentowanie: f64,
    kwota_kredytowania: f64,
    okres_kredytowania: u64,
    calkowity_koszt_nieruchomosci: f64,
    koszt_kredytu: f64,
    mapa_kosztow: BTreeMap<String, KosztKoncowy>,
    mapa_rat: BTreeMap<u64, Rata>,
    nadplaty: Nadplaty,
}

impl Kalkulator {
    pub fn new(dto: Kredyt) -> Self {
        let kwota_kredytowania = dto.wartosc_hipoteki - dto.wartosc_hipoteki * dto.wklad_wlasny / 100.0;
        let mapa_kosztow = Self::mapa_kosztow(&dto);

        let mapa_rat = MapaRat::new(kwota_kredytowania, &dto).mapa_rat().clone();

        let koszt_kredytu = Self::koszt_kredytu_internal(&mapa_rat, &mapa_kosztow, dto.okres_kredytowania, None);
        let calkowity_koszt_nieruchomosci = dto.wartosc_hipoteki + koszt_kredytu;

        Self {
            nazwa: dto.nazwa,
            wartosc_hipoteki: dto.wartosc_hipoteki,
            wklad_wlasny: dto.wklad_wlasny,
            oprocentowanie: dto.oprocentowanie,
            kwota_kredytowania,
            okres_kredytowania: dto.okres_kredytowania,
            nadplaty: dto.nadplaty,
            calkowity_koszt_nieruchomosci,
            koszt_kredytu,
            mapa_kosztow,
            mapa_rat,
        }
    }

    pub fn koszt_kredytu(&self, numer_raty: impl Into<Option<u64>>) -> f64 {
        Self::koszt_kredytu_internal(&self.mapa_rat, &self.mapa_kosztow, self.okres_kredytowania, numer_raty)
    }

    fn koszt_kredytu_internal(
        mapa_rat: &BTreeMap<u64, Rata>,
        mapa_kosztow: &BTreeMap<String, KosztKoncowy>,
        okres_kredytowania: u64,
        numer_raty: impl Into<Option<u64>>
    ) -> f64 {
        let numer_raty = numer_raty.into().unwrap_or(okres_kredytowania);

        mapa_rat.range(0..numer_raty).map(|(_, v)| v.odsetki).sum::<f64>()
        + mapa_kosztow.iter().map(|(_, k)| k.oblicz(0.0, numer_raty).wartosc()).sum::<f64>()
    }

    pub fn procent_hipoteki(&self, numer_raty: impl Into<Option<u64>>) -> f64 {
        let numer_raty = numer_raty.into().unwrap_or(self.okres_kredytowania);
        let wplacony_kapital: f64 = self.mapa_rat.range(0..numer_raty).map(|(_, v)| v.kapital + v.nadplata).sum();
        let splacony_procent = wplacony_kapital * 100.0 / self.kwota_kredytowania;

        self.wklad_wlasny + splacony_procent
    }

    fn mapa_kosztow(dto: &Kredyt) -> BTreeMap<String, KosztKoncowy> {
        let mut koszty: BTreeMap<String, KosztKoncowy> = dto.koszty.iter().map(|k| (k.nazwa(), k.oblicz(dto.wartosc_hipoteki - dto.wartosc_hipoteki * dto.wklad_wlasny / 100.0, dto.okres_kredytowania))).collect();
        let pcc = dto.wartosc_hipoteki * 2.0 / 100.0;
        let pcc = KosztKoncowy::new(
                Koszt::builder()
                    .nazwa("PCC")
                    .wartosc(2.0)
                    .operator(Operator::Procent)
                    .okres(Okres::Jednorazowy)
                    .build(),
                pcc
            );

        koszty.insert("PCC".to_string(), pcc);

        koszty
    }
}

impl fmt::Display for Kalkulator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Koszt nieruchomosci w banku `{}` wynosi: {:.2} zl", self.nazwa, self.calkowity_koszt_nieruchomosci)?;

        writeln!(f, "Koszty:")?;
        for (_, koszt) in self.mapa_kosztow.iter() {
            writeln!(f, "    * {}", koszt)?;
        }

        writeln!(f, "Raty:")?;

        for (i, rata) in self.mapa_rat.iter().step_by(1) {
            writeln!(f, "    * Skladowe raty (w {}. miesiacu kredytu): {:.2}", (i + 1) , rata)?;
        }
        writeln!(f, "{}", self.nadplaty)?;

        for rok in vec![3, 5, 7, 10, 15] {
            let okres = rok * 12 + 1;
            let procent_hipoteki = self.procent_hipoteki(okres);
            let kapital = self.wartosc_hipoteki * procent_hipoteki / 100.0;
            writeln!(f, "Koszt kredytu po {} latach: {:.2}zl", rok, self.koszt_kredytu(okres))?;
            writeln!(f, "Procent hipoteki po {} latach: {:.2}% ({:.2}zl)", rok, procent_hipoteki, kapital)?;
        }

        dbg!(self.mapa_rat.values().map(|v| v.odsetki).sum::<f64>());
        dbg!(self.mapa_rat.values().map(|v| v.kapital + v.nadplata).sum::<f64>());

        writeln!(f, "Calkowity koszt kredytu {:.2}zl", self.koszt_kredytu(None))?;

        writeln!(f, "")
    }
}

pub fn oferta(path: &str) {
    let data = std::fs::read_to_string(path).unwrap();
    let kredyt: Kredyt = serde_json::from_str(&data).unwrap();
    let kalkulator = Kalkulator::new(kredyt);

    println!("{}", kalkulator);
}

#[cfg(test)]
mod tests {
    use super::*;



}