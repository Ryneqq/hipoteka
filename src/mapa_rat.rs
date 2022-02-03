use std::{collections::BTreeMap, fmt};

use crate::{kredyt::Kredyt, Nadplaty};

pub struct MapaRat {
    mapa: BTreeMap<u64, Rata>
}

impl MapaRat {
    pub fn new(kwota_kredytowania: f64, dto: &Kredyt) -> Self {
        let mapa = KalkulatorRaty::new(kwota_kredytowania, dto.oprocentowanie, dto.okres_kredytowania, dto.nadplaty.clone()).mapa_rat();

        Self { mapa }
    }

    pub fn mapa_rat(&self) -> &BTreeMap<u64, Rata> {
        &self.mapa
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

        let mut retval: BTreeMap<u64, Rata> = (0..self.okres_kredytowania).map(|numer_raty| {
            let odsetki = kapital_do_splaty * oprocentowanie * 30.4375 / 365.0;
            let kapital = (aktualna_rata.wartosc() - odsetki).clamp(0.0, kapital_do_splaty);
            // let nadplata = self.nadplaty.wartosc(numer_raty);
            // let pozostaly_okres_kredytowania = self.okres_kredytowania - numer_raty;
            // musisz podzielic kapital na wszystkie miesiace
            let rata = Rata { kapital, odsetki, nadplata: 0.0 };

            aktualna_rata = rata;
            dbg!(aktualna_rata.wartosc());
            kapital_do_splaty = (kapital_do_splaty - aktualna_rata.kapital).clamp(0.0, self.kwota_kredytowania);
            // kapital_do_splaty = (kapital_do_splaty - aktualna_rata.kapital - nadplata).clamp(0.0, self.kwota_kredytowania);
            // ulga_od_nadplaty = nadplata / pozostaly_okres_kredytowania as f64;
            // aktualna_rata.ulga(ulga_od_nadplaty);

            (numer_raty, rata)
        })
        .collect();

        dbg!(retval.values().map(|rata| rata.kapital()).sum::<f64>());

        for numer_raty in 0..self.okres_kredytowania {
                let nadplata = self.nadplaty.wartosc(numer_raty);
                dbg!(nadplata);
                let pozostaly_okres_kredytowania = self.okres_kredytowania - numer_raty - 1;
                let ulga_od_nadplaty = nadplata / pozostaly_okres_kredytowania as f64;
                dbg!(ulga_od_nadplaty);
                let kapital_splacony = retval.values()
                    .map(|rata| rata.kapital())
                    .take(numer_raty as usize + 1)
                    .sum::<f64>();
                dbg!(kapital_splacony);
                let kapital_do_splaty = (self.kwota_kredytowania - kapital_splacony).max(0.0);

                retval.get_mut(&numer_raty).map(|rata| {
                    rata.nadplata(nadplata);
                    rata.odsetki = kapital_do_splaty * oprocentowanie * 30.4375 / 365.0;
                });

            for (_, rata) in retval.iter_mut().skip(numer_raty as usize + 1) {
                rata.ulga(ulga_od_nadplaty);
            }
        }

        retval
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

    pub fn kapital(&self) -> f64 { self.kapital + self.nadplata }
}

impl fmt::Display for Rata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2} = kapital: {:.2}, odsetki: {:.2}, nadplata: {:.2}", self.wartosc() + self.nadplata, self.kapital, self.odsetki, self.nadplata)
    }
}