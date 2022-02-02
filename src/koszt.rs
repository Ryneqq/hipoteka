use std::{fmt, ops::Deref};

use serde::{Serialize, Deserialize};
use typed_builder::TypedBuilder;

#[derive(Clone, Serialize, Deserialize, TypedBuilder)]
pub struct Koszt {
    #[serde(rename = "nazwa")]
    #[builder(setter(into))]
    nazwa: String,

    #[serde(rename = "wartosc")]
    wartosc: f64,

    #[serde(rename = "operator")]
    operator: Operator,

    #[serde(rename = "okres")]
    okres: Okres,

    #[serde(rename = "okresow")]
    #[builder(default, setter(strip_option))]
    okresow: Option<u64>,
}

impl Koszt {
    pub fn nazwa(&self) -> String {
        self.nazwa.to_string()
    }

    pub fn oblicz(&self, wartosc_bazowa: f64, okres: u64) -> KosztKoncowy {
        let wartosc = match self.operator {
            Operator::Procent => self.wartosc * wartosc_bazowa / 100.0,
            Operator::Stala => self.wartosc
        };

        let wartosc = match self.okres {
            Okres::Jednorazowy => {
                wartosc
            },
            Okres::Miesieczny => {
                wartosc * self.okresow.unwrap_or(okres) as f64
            }
            Okres::Roczny => {
                wartosc * (okres / 12) as f64
            }
        };

        KosztKoncowy::new(self.clone(), wartosc)
    }

    pub fn okres(&self) -> Okres {
        self.okres
    }

    pub fn okresow(&self) -> Option<u64> {
        self.okresow
    }

    pub fn wartosc(&self) -> f64 {
        self.wartosc
    }
}

impl fmt::Display for Koszt {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}: {}{} {}", self.nazwa, self.wartosc, self.operator, self.okres)
    }
}

pub struct KosztKoncowy {
    data: Koszt,
    total: f64,
}

impl KosztKoncowy {
    pub fn new(data: Koszt, total: f64) -> KosztKoncowy {
        Self { data, total }
    }

    pub fn total(&self) -> f64 {
        self.total
    }
}

impl Deref for KosztKoncowy {
    type Target = Koszt;

     fn deref(&self) -> &Self::Target {
         &self.data
     }
}

impl fmt::Display for KosztKoncowy {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}, total: {:.2}zl", self.data, self.total)
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Operator {
    Procent,
    Stala
}

impl fmt::Display for Operator {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operator::Procent => write!(fmt, "%"),
            Operator::Stala => write!(fmt, "zl")
        }
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Okres {
    Jednorazowy,
    Miesieczny,
    Roczny,
}

impl fmt::Display for Okres {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Okres::Jednorazowy => write!(fmt, "jednorazowo"),
            Okres::Miesieczny => write!(fmt, "miesiecznie"),
            Okres::Roczny => write!(fmt, "rocznie"),
        }
    }
}