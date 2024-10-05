use plotters::prelude::*;
use transfer_matrix::{Layer, LayerStack};

fn main() {
    let mut stack: LayerStack = LayerStack::new();

    // Initialize the stack.
    stack.add_layer(Layer::new(1.00, 0.00));
    stack.add_layer(Layer::new(1.52, 400.));
    stack.add_layer(Layer::new(1.00, 0.00));

    let samples: Vec<(f64, (f64, f64))> = (400..1700)
        .map(|x| x as f64)
        .map(|wl| (wl, stack.transfer(wl).unwrap()))
        .collect();

    let drawing_area = BitMapBackend::new("images/1.0-single-layer-400nm-1700nm.png", (1280, 720))
        .into_drawing_area();

    drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&drawing_area)
        .caption(
            "Reflectance of Single Layer (n=1.52)",
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
