use ndarray::{ Axis,Array, Array1, Array3, Array4, ArrayView1, ArrayView3, ArrayView4, ArrayViewMut1, s};
use ndarray::parallel::prelude::*;
// use std::iter::FromIterator;
// use rayon::prelude::*;
// use std::convert::TryInto;
// use ndarray::Zip;

pub fn screen<'a>(color1: ArrayView1<u8>, color2: ArrayView1<u8>,color_out:&'a mut Array1<u8>) -> &'a Array1<u8>{


    let r = 255.0 - 255.0 * (1.0 - f32::from(color1[0])/255.0) * (1.0 - f32::from(color2[0])/255.0);
    let r_out = r.round() as u8;
    let g = 255.0 - 255.0 * (1.0 - f32::from(color1[1])/255.0) * (1.0 - f32::from(color2[1])/255.0);
    let g_out = g.round() as u8;
    let b = 255.0 - 255.0 * (1.0 - f32::from(color1[2])/255.0) * (1.0 - f32::from(color2[2])/255.0);
    let b_out = b.round() as u8;
    let alpha = 255 - ((255 - color1[3]) * (255 - color2[3]) / 255);
    let alpha_out = alpha as u8;


    // let mut color : Array1<u8> = Array1::<u8>::zeros(4);
    color_out[[0]] = r_out;
    color_out[[1]] = g_out;
    color_out[[2]] = b_out;
    color_out[[3]] = alpha_out;

    // let mut color: Array1<u8> = Array1::<u8>::zeros(4);
    // let mut i = 0;
    // for col in &*color_out {
    //     color[i] = col.clone();
    //     i = i + 1;
    // }

    // let mut color = array![ r_out, g_out, b_out,alpha_out];
    // let color = ArrayViewMut1::<u8>::from(vec![r_out, g_out, b_out, alpha]);
    // color[0] = r_out;
    // color[1] = g_out;
    // color[2] = b_out;
    // color[3] = alpha;
    return color_out
}

pub fn screen_frame<'b>(channel1: ArrayView3<u8>, channel2:ArrayView3<u8>) -> Array3<u8>{

    // let mut frame = Array3::<u8>::zeros(channel1.dim());
    let mut frame = Array3::<u8>::zeros(channel1.dim());
    for (row_index, mut row) in frame.axis_iter_mut(Axis(0)).enumerate(){
        for (column_index, mut pixel) in row.axis_iter_mut(Axis(0)).enumerate(){
            let mut color_out : Array1<u8> = Array1::<u8>::zeros(4);
            pixel.assign(screen(channel1.slice(s![row_index, column_index, ..]), channel2.slice(s![row_index, column_index, ..]),&mut color_out));
        }
    }
    return frame
}

pub fn screen_stack(stack1: ArrayView4<u8>, stack2: ArrayView4<u8>) -> Array4<u8>{

    // let mut stack = Array4::<u8>::zeros(stack1.dim());
    // let mut stack = Array4::<u8>::zeros(stack1.dim()).axis_iter(Axis(0)).into_par_iter();


    // Zip::from(&mut stack).and(&stack1).and(&stack2).par_apply(|stack, &stack1, &stack2|{
    //     stack = screen_frame(stack1, stack2);
    // });

    // let frames: Vec<i32> = (0..length).collect();
    let frames: Vec<usize> = (0..stack1.dim().0).collect();
    // let frames = Vec::from([0 as usize..length as usize]);

    // let mut out = vec![vec![vec![vec![0, 4]; stack1.dim().2]; stack1.dim().1]; stack1.dim().0];
    // let mut out = vec![0, stack1.dim().0];

    let out : Vec<_> = frames.par_iter().map(|img|{
        let index = img.clone();
        // stack.slice_mut(s![img, .., .., ..]).assign(&screen_frame(stack1.slice(s![img, .., .., ..]), stack2.slice(s![img, .., .., ..])));
        // screen_frame(stack1.slice(s![img, .., .., ..]), stack2.slice(s![img, .., .., ..]))
        screen_frame(stack1.slice(s![index, .., .., ..]), stack2.slice(s![index, .., .., ..]))
    }).collect();

    // convert this Vec<Array3<u8>> to an Array4<u8>
    let inner_shape = out[0].dim();
    let shape = (out.len(), inner_shape.0, inner_shape.1, inner_shape.2);
    let flat: Vec<u8> = out.iter().flatten().cloned().collect();
    let arr = Array4::from_shape_vec(shape, flat).unwrap(); //?

    return arr
}
// TRY LIKE THIS:
// let nested: Vec<Array2<i32>> = vec![
//     array![[1, 2, 3], [4, 5, 6]],
//     array![[7, 8, 9], [10, 11, 12]],
// ];
// let inner_shape = nested[0].dim();
// let shape = (nested.len(), inner_shape.0, inner_shape.1);
// let flat: Vec<i32> = nested.iter().flatten().cloned().collect();
// let arr = Array3::from_shape_vec(shape, flat)?;
