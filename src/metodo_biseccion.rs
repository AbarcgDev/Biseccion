pub mod intervalo;
pub mod iteracion;

use iteracion::Iteracion;

use self::intervalo::Intervalo;

pub fn biseccion(
    intervalo: Intervalo,
    funcion: impl Fn(f64) -> f64,
    precision: f64,
) -> Vec<Iteracion> {
    let mut iteraciones: Vec<Iteracion> = vec![];
    let mut iteracion = Iteracion {
        intervalo,
        numero: 0,
        aproximacion: intervalo.0,
        error: 100.0,
    };
    loop {
        let punto_medio = iteracion.intervalo().punto_medio();
        let intervalo_sig;

        if funcion(iteracion.intervalo().0) * funcion(punto_medio) < 0.0 {
            intervalo_sig = Intervalo(iteracion.intervalo().0, punto_medio);
        } else {
            intervalo_sig = Intervalo(punto_medio, iteracion.intervalo().1);
        }

        let nueva_iteracion = Iteracion {
            intervalo: intervalo_sig,
            numero: iteracion.numero + 1,
            aproximacion: punto_medio,
            error: error_abs(punto_medio, iteracion.aproximacion()),
        };

        if error_abs(punto_medio, iteracion.aproximacion()) > precision {
            iteracion = nueva_iteracion;
            iteraciones.push(nueva_iteracion);
        } else {
            iteraciones.push(nueva_iteracion);
            break;
        }
    }

    iteraciones
}

pub fn error_abs(pn: f64, pn_1: f64) -> f64 {
    (pn - pn_1).abs()
}
