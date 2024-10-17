use transfer_matrix::{Layer, LayerStack};

fn main() {
    let mut stack: LayerStack = LayerStack::new();

    // Initialize the stack.
    stack.add_layer(Layer::new_air(0.00));
    stack.add_layer(Layer::new_glass(400.));
    stack.add_layer(Layer::new_air(0.00));

    let (refl, trns) = stack.transfer(500.0).unwrap();

    println!("--------------------------------");
    println!("refl {:?}, trns {:?}", refl, trns);
}
