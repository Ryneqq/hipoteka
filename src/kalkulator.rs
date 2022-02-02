use std::{collections::BTreeMap, fmt, cmp};

use crate::{KosztKoncowy, kredyt::Kredyt, Koszt, Operator, Okres, Nadplaty};

// TODO % posiadanej hipoteki po x latach
// TODO nadplaty

pub struct Kalkulator {
    nazwa: String,
    wartosc_hipoteki: f64,
    wklad_wlasny: f64,
    oprocentowanie: f64,
    kwota_kredytowania: f64,
    okres_kredytowania: u64,
    rata_poczatkowa: Rata,
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
        let nadplaty = dto.nadplaty;

        let rata_poczatkowa = Rata::new(kwota_kredytowania, dto.oprocentowanie, dto.okres_kredytowania);
        let mapa_rat = KalkulatorRaty::new(kwota_kredytowania, dto.oprocentowanie, dto.okres_kredytowania, nadplaty.clone()).mapa_rat();

        let koszt_kredytu = Self::koszt_kredytu_internal(&mapa_rat, &mapa_kosztow, dto.okres_kredytowania, None);
        let calkowity_koszt_nieruchomosci = dto.wartosc_hipoteki + koszt_kredytu;

        Self {
            nazwa: dto.nazwa,
            wartosc_hipoteki: dto.wartosc_hipoteki,
            wklad_wlasny: dto.wklad_wlasny,
            oprocentowanie: dto.oprocentowanie,
            kwota_kredytowania,
            okres_kredytowania: dto.okres_kredytowania,
            nadplaty,
            rata_poczatkowa,
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

pub struct KalkulatorRaty {
    rata_poczatkowa: Rata,
    kwota_kredytowania: f64,
    oprocentowanie: f64,
    okres_kredytowania: u64,
    nadplaty: Nadplaty,
}

impl KalkulatorRaty {
    pub fn new(kwota_kredytowania: f64, oprocentowanie: f64, okres_kredytowania: u64, nadplaty: Nadplaty) -> Self {
        let rata_poczatkowa = Rata::new(kwota_kredytowania, oprocentowanie, okres_kredytowania);

        Self {
            rata_poczatkowa,
            kwota_kredytowania,
            oprocentowanie,
            okres_kredytowania,
            nadplaty
        }
    }

    pub fn mapa_rat(&self) -> BTreeMap<u64, Rata> {
        let oprocentowanie = self.oprocentowanie / 100.0;
        let mut kapital_do_splaty = self.kwota_kredytowania;
        let mut aktualna_rata = self.rata_poczatkowa;
        let mut ulga_od_nadplaty = 0.0;

        (0..self.okres_kredytowania).map(|numer_raty| {
            let odsetki = kapital_do_splaty * oprocentowanie * 30.4375 / 365.25;
            let nadplata = self.nadplaty.wartosc(numer_raty);
            let pozostaly_okres_kredytowania = self.okres_kredytowania - numer_raty;
            let kapital = (aktualna_rata.wartosc() - odsetki).clamp(0.0, kapital_do_splaty);
            let rata = Rata { kapital, odsetki, nadplata };

            aktualna_rata = rata;
            kapital_do_splaty = (kapital_do_splaty - aktualna_rata.kapital - nadplata).clamp(0.0, self.kwota_kredytowania);
            ulga_od_nadplaty = nadplata / pozostaly_okres_kredytowania as f64;
            aktualna_rata.ulga(ulga_od_nadplaty);

            (numer_raty, rata)
        })
        .collect()
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Rata {
    pub kapital: f64,
    pub odsetki: f64,
    pub nadplata: f64,
}

impl Rata {
    pub fn wartosc(&self) -> f64 { self.kapital + self.odsetki }

    pub fn new(kwota_kredytowania: f64, oprocentowanie: f64, okres_kredytowania: u64) -> Self {
        let n = 12.0; // liczba rat w ciągu roku
        let okres_kredytowania = okres_kredytowania as f64;
        let oprocentowanie = oprocentowanie / 100.0;
        let rata = kwota_kredytowania * oprocentowanie / (n * (1.0 - (n / (n + oprocentowanie)).powf(okres_kredytowania)));
        let odsetki = kwota_kredytowania * oprocentowanie * 30.4375 / 365.25;
        let kapital = rata - odsetki;

        Self {
            kapital,
            odsetki,
            nadplata: 0.0
        }
    }

    pub fn ulga(&mut self, nadplata: f64) {
        self.kapital = (self.kapital - nadplata).max(0.0);
    }

    pub fn nadplata(&mut self, nadplata: f64) {
        self.nadplata = nadplata
    }
}

impl fmt::Display for Rata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2} = kapital: {:.2}, odsetki: {:.2}, nadplata: {:.2}", self.wartosc() + self.nadplata, self.kapital, self.odsetki, self.nadplata)
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