use num::complex::Complex;
use plotters::prelude::*;
use transfer_matrix::{Layer, LayerStack};

fn main() {
    let mut stack: LayerStack = LayerStack::new();

    let tio2 = |wavelength_nm: f64| -> Complex<f64> { Complex::new(2.08, 0.0) };
    let sio2 = |wavelength_nm: f64| -> Complex<f64> { Complex::new(1.46, 0.0) };
    let ag = |wavelength_nm: f64| -> Complex<f64> { Complex::new(1.46, 0.0) };

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
        BitMapBackend::new("images/2.0-bandpass-400nm-1700nm.png", (1280, 720)).into_drawing_area();

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
