use num_complex::Complex;
use std::ops::Add;
use crate::math::interpolate::linear_interpolate;


pub fn simple_resampling(signal_in: &[Complex<f32>], freq_arr: &[f32], out_signal: &mut Vec<Complex<f32>>,
                                freq_conv: &mut Vec<f32>, freq_conv_image: &mut Vec<f32>, orientited_signal_len: usize) -> f32 {
    out_signal.clear();
    freq_conv_image.clear();
    let mut freq_avg: f32 = 0.0;
    for i in 0..freq_arr.len(){
        freq_avg = freq_avg + freq_arr[i];
    }
    freq_avg = freq_avg/(freq_arr.len() as f32);


    let mut iters_predict = 0;
    let mut temp: f32 = 0.0;
    let mut temp1: f32;
    while temp<=(signal_in.len() as f32-1.0){
        temp1 =  freq_avg / linear_interpolate(freq_arr, temp);
        temp = temp + temp1;
        iters_predict = iters_predict + 1;
    }
    freq_avg = num_traits::float::Float::sqrt(freq_avg*freq_avg/ ((orientited_signal_len as f32) / (iters_predict as f32))/((orientited_signal_len as f32) / (iters_predict as f32)));

    temp = 0.0;
    while temp<=(signal_in.len() as f32-1.0){
        temp1 =  freq_avg / linear_interpolate(freq_arr, temp);
        out_signal.push(linear_interpolate(signal_in, temp));
        freq_conv_image.push(temp1 * linear_interpolate(freq_conv, temp));
        temp = temp + temp1;
    }



    std::mem::swap(freq_conv_image, freq_conv);
    freq_conv_image.clear();

    freq_avg
}

pub fn back_resampling <T> (arrin: &[T], arrout: &mut Vec<T>, freq_conv: &[f32], orientited_signal_len: usize)
    where T: Copy + std::ops::Mul<f32> + Add + Add<Output = T> + std::ops::Mul<f32, Output = T> + From<<<T as std::ops::Mul<f32>>::Output as Add>::Output>, <T as std::ops::Mul<f32>>::Output: Add  + std::fmt::Display{
    arrout.clear();
    let mut temp = 0.0;
    let mut temp1: f32;
    let mut sum = 0.0;
    for i in 0..freq_conv.len(){
        sum = sum + freq_conv[i];
    }

    let mut iter_predict = 0;
    while temp <= (arrin.len() as f32 - 1.0){
        temp1 = 1.0 / linear_interpolate::<f32>(freq_conv, temp) as f32;
        if 0.0 == linear_interpolate::<f32>(freq_conv, temp) as f32{
            temp = temp + 1.0;
        }
        /*else{
            if temp1<(arrin.len() as f32){
                arrout.push(linear_interpolate::<T>(arrin, temp));
                temp = temp + temp1;
            }
            else{
                temp = temp + temp1;
            }
        }*/
        else{
            temp = temp + temp1;
            iter_predict = iter_predict+1;
        }
    }
    let mut one = 1.0;
    //one = num_traits::float::Float::sqrt(one /((orientited_signal_len as f32) / (iter_predict as f32))/((orientited_signal_len as f32) / (iter_predict as f32)));
    one = one/((orientited_signal_len as f32) / (iter_predict as f32));
    temp = 0.0;

    while temp <= (arrin.len() as f32 - 1.0){
        temp1 = one / linear_interpolate::<f32>(freq_conv, temp) as f32;
        if 0.0 == linear_interpolate::<f32>(freq_conv, temp) as f32{
            temp = temp + 1.0;
        }
        /*else{
            if temp1<(arrin.len() as f32){
                arrout.push(linear_interpolate::<T>(arrin, temp));
                temp = temp + temp1;
            }
            else{
                temp = temp + temp1;
            }
        }*/
        else{
            arrout.push(linear_interpolate::<T>(arrin, temp));
            temp = temp + temp1;
        }
    }
}

