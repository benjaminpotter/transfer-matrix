use interp::{interp, InterpMode};
use num::complex::Complex;
use plotters::prelude::*;
use transfer_matrix::{Layer, LayerStack};

fn main() {
    let mut stack: LayerStack = LayerStack::new();

    let tio2 = |wavelength_nm: f64| -> Complex<f64> { Complex::new(2.08, 0.0) };
    let sio2 = |wavelength_nm: f64| -> Complex<f64> { Complex::new(1.46, 0.0) };
    let ag = |wavelength_nm: f64| -> Complex<f64> {
        // FIXME this obviously isn't the most performant approach
        let wl_ns = vec![
            187.90, 191.60, 195.30, 199.30, 203.30, 207.30, 211.90, 216.40, 221.40, 226.20, 231.30,
            237.10, 242.60, 249.00, 255.10, 261.60, 268.90, 276.10, 284.40, 292.40, 300.90, 310.70,
            320.40, 331.50, 342.50, 354.20, 367.90, 381.50, 397.40, 413.30, 430.50, 450.90, 471.40,
            495.90, 520.90, 548.60, 582.10, 616.80, 659.50, 704.50, 756.00, 821.10, 892.00, 984.00,
            1088.00, 1216.00, 1393.00, 1610.00, 1937.00,
        ];
        let ns = vec![
            1.07, 1.10, 1.12, 1.14, 1.15, 1.18, 1.20, 1.22, 1.25, 1.26, 1.28, 1.28, 1.30, 1.31,
            1.33, 1.35, 1.38, 1.41, 1.41, 1.39, 1.34, 1.13, 0.81, 0.17, 0.14, 0.10, 0.07, 0.05,
            0.05, 0.05, 0.04, 0.04, 0.05, 0.05, 0.05, 0.06, 0.05, 0.06, 0.05, 0.04, 0.03, 0.04,
            0.04, 0.04, 0.04, 0.09, 0.13, 0.15, 0.24,
        ];

        let wl_ks = vec![
            187.90, 191.60, 195.30, 199.30, 203.30, 207.30, 211.90, 216.40, 221.40, 226.20, 231.30,
            237.10, 242.60, 249.00, 255.10, 261.60, 268.90, 276.10, 284.40, 292.40, 300.90, 310.70,
            320.40, 331.50, 342.50, 354.20, 367.90, 381.50, 397.40, 413.30, 430.50, 450.90, 471.40,
            495.90, 520.90, 548.60, 582.10, 616.80, 659.50, 704.50, 756.00, 821.10, 892.00, 984.00,
            1088.00, 1216.00, 1393.00, 1610.00, 1937.00,
        ];
        let ks = vec![
            1.21, 1.23, 1.25, 1.28, 1.30, 1.31, 1.32, 1.34, 1.34, 1.34, 1.36, 1.37, 1.38, 1.39,
            1.39, 1.39, 1.37, 1.33, 1.26, 1.16, 0.96, 0.62, 0.39, 0.83, 1.14, 1.42, 1.66, 1.86,
            2.07, 2.27, 2.46, 2.66, 2.87, 3.09, 3.32, 3.59, 3.86, 4.15, 4.48, 4.84, 5.24, 5.73,
            6.31, 6.99, 7.79, 8.83, 10.10, 11.85, 14.08,
        ];
        let n = interp(&wl_ns, &ns, wavelength_nm, &InterpMode::FirstLast);
        let k = interp(&wl_ks, &ks, wavelength_nm, &InterpMode::FirstLast);
        Complex::new(n, k)
    };

    // Initialize the stack.
    stack.add_layer(Layer::new_glass(1000.));
    stack.add_layer(Layer::new(sio2, 100.));
    stack.add_layer(Layer::new(ag, 30.));
    stack.add_layer(Layer::new(sio2, 100.));
    stack.add_layer(Layer::new(tio2, 50.));

    let samples: Vec<(f64, (f64, f64))> = (400..1700)
        .map(|x| x as f64)
        .map(|wl| (wl, stack.transfer(wl).unwrap()))
        .collect();

    let drawing_area =
        SVGBackend::new("images/2.0-bandpass-400nm-1700nm.svg", (1280, 720)).into_drawing_area();

    drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&drawing_area)
        .caption(
            "Reflectance of Bandpass TiO2 and SiO2",
            ("serif", 15).into_font(),
        )
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(20)
        .build_cartesian_2d(400.0..1700.0, 0.0..1.0)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart
        .draw_series(LineSeries::new(
            samples.iter().cloned().map(|(wl, (refl, _))| (wl, refl)),
            &BLACK,
        ))
        .unwrap();
}
