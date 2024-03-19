use super::intervalo::{self, Intervalo};

#[derive(Debug, Copy, Clone)]
pub struct Iteracion {
    pub intervalo: Intervalo,
    pub numero: u64,
    pub aproximacion: f64,
    pub error: f64,
}

impl Iteracion {
    pub fn new(intervalo: Intervalo, numero: u64, aproximacion: f64, error: f64) -> Iteracion {
        Iteracion {
            intervalo,
            numero,
            aproximacion,
            error,
        }
    }

    pub fn intervalo(&self) -> &Intervalo {
        &self.intervalo
    }

    pub fn aproximacion(&self) -> f64 {
        self.aproximacion
    }
}
