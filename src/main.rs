mod screen_module;

fn main() {
    // This is a theoretical mini image
    let img_in = vec!(vec![vec![ 98, 100,   0, 255], vec![100,  12,   0, 255], vec![180, 234, 100, 255]],
        vec![vec![210, 189, 120, 255], vec![172,   0,  82, 255], vec![100, 100, 100, 255]],
        vec![vec![ 72, 172,  12, 255], vec![ 92, 139,   0, 255], vec![ 23,  43,  30, 255]]);

    let img_in2 = vec!(vec![vec![ 98, 100,   0, 255], vec![100,  12,   0, 255], vec![180, 234, 100, 255]],
        vec![vec![210, 189, 120, 255], vec![172,   0,  82, 255], vec![100, 100, 100, 255]],
        vec![vec![ 72, 172,  12, 255], vec![ 92, 139,   0, 255], vec![ 23,  43,  30, 255]]);

    println!("{:?}", &img_in[1][1][1]);

    // let color = screen_module::screen(vec!(100, 0, 10, 255), vec!(0,200,30, 255));
    // let img = screen_module::screen_frame(img_in, img_in2);



    println!("{:?}", img_in);
    println!("{:?}", img_in2)
}
