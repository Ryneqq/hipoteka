use std::{fmt, cmp};
use serde::{Serialize, Deserialize};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Nadplaty(Vec<Nadplata>);

impl Nadplaty {
    pub fn wartosc(&self, numer_raty: u64) -> f64 {
        self.0
            .iter()
            .filter(|n| {
                if n.po_okresie {
                    n.to == numer_raty
                } else {
                    n.from <= numer_raty && n.to > numer_raty
                }
            })
            .map(|n| n.wartosc())
            .sum()
    }
}

impl fmt::Display for Nadplaty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0.is_empty() {
            writeln!(f, "Nadplaty: Brak")?;
        } else {
            writeln!(f, "Nadplaty:")?;
        }

        for n in self.0.iter() {
            if n.po_okresie {
                writeln!(f, "    * Po okresie {} - {}, sumaryczna wartosc: {}, miesieczna: {}", n.from, n.to, n.wartosc(), n.wartosc)?;
            } else {
                writeln!(f, "    * W okresie {} - {}, miesieczna wartosc nadplaty: {}", n.from, n.to, n.wartosc())?;
            }
        }

        Ok(())
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Nadplata {
    pub wartosc: f64,
    #[serde(rename = "od")]
    pub from: u64,
    #[serde(rename = "do")]
    pub to: u64,
    pub po_okresie: bool
}

impl Nadplata {
    pub fn wartosc(&self) -> f64 {
        if self.po_okresie {
            cmp::max(0, self.to - self.from) as f64 * self.wartosc
        } else {
            self.wartosc
        }
    }
}

impl fmt::Display for Nadplata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.wartosc())
    }
}


