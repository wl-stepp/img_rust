mod screen_module;
use ndarray::Array4;

fn main() {
    // This is a theoretical mini image
    let stack1 = Array4::<u8>::ones((10, 100, 100, 4));
    let stack2 = Array4::<u8>::ones((10, 100, 100, 4));

    let stack_out = screen_module::screen_stack(stack1.view(), stack2.view());

    println!("{:?}", stack_out);
}
