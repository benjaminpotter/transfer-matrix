use ndarray::{arr2, Array2};
use num::complex::Complex;
use std::f64::consts::TAU;

pub struct Layer {
    refractive_index: f64,
    length_nm: f64,
}

impl Layer {
    pub fn new(refractive_index: f64, length_nm: f64) -> Self {
        Layer {
            refractive_index: refractive_index,
            length_nm: length_nm,
        }
    }
}

impl Layer {
    fn transmit(&self, next: &Layer) -> Array2<Complex<f64>> {
        let g: Complex<f64> = Complex::new(next.refractive_index / self.refractive_index, 0.0);

        // Assume incident angle is aligned with normal and TE polarization.
        arr2(&[[1.0 + g, 1.0 - g], [1.0 - g, 1.0 + g]])
    }

    fn propagate(&self, wavelength_nm: f64) -> Array2<Complex<f64>> {
        let kz: f64 = TAU / wavelength_nm * self.refractive_index;

        // Ignore complex part of the propagation.
        arr2(&[
            [
                Complex::new(0.0, -kz * self.length_nm).exp(),
                Complex::new(0.0, 0.0),
            ],
            [
                Complex::new(0.0, 0.0),
                Complex::new(0.0, kz * self.length_nm).exp(),
            ],
        ])
    }
}

pub struct LayerStack {
    layers: Vec<Layer>,
}

impl LayerStack {
    pub fn new() -> Self {
        LayerStack { layers: Vec::new() }
    }

    pub fn add_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
    }

    pub fn transfer(&self, wavelength_nm: f64) -> Option<(f64, f64)> {
        if self.layers.is_empty() {
            return None;
        }

        // Initialize transfer matrix (tm) as identity.
        let mut tm: Array2<Complex<f64>> = arr2(&[
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
        ]);

        for window in self.layers.windows(2) {
            let left = &window[0];
            let right = &window[1];

            tm = tm.dot(&left.propagate(wavelength_nm));
            tm = tm.dot(&left.transmit(right));
        }

        // Compute the reflectance and transmittance of the stack.
        let refl = (tm[[1, 0]] / tm[[0, 0]]).norm().powf(2.0);
        let trns = (1.0 / tm[[0, 0]].norm()).powf(2.0);

        Some((refl, trns))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn air_glass_air() {
        let mut stack: LayerStack = LayerStack::new();

        // Initialize the stack.
        stack.add_layer(Layer::new(1.00, 0.00));
        stack.add_layer(Layer::new(1.52, 400.));
        stack.add_layer(Layer::new(1.00, 0.00));

        let (refl, trns) = stack.transfer(500.0).unwrap();

        assert_relative_eq!(refl, 0.150, epsilon = 0.01);
    }
}
