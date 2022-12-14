use num_complex::Complex;

pub fn generate_simple_mask (mask_len: usize, pow: f32, mask_buffer: &mut [Complex<f32>]){
    let powf32: f32 = std::convert::Into::into(pow);
    let mut temp = mask_len / 2;
    let mut mn = crate::math::instrumental_math::pow_sum (temp, pow);
    mn = mn * 2.0;
    let mn_temp:f32 = mn as f32;
    mn = 1.0 /(mn_temp + (mask_len % 2) as f32  *  ((mask_len / 2 + 1) as f32).powf(powf32));
    if mask_len % 2 == 1 {
        temp = temp + 1;
    }
    for i in 0..mask_buffer.len(){
        mask_buffer[i].im= 0.0;
        mask_buffer[i].re = 0.0;
    }
    for i in 0..temp {
        mask_buffer[i].re = mn * (i as f32 + 1.0).powf(powf32);
        mask_buffer[mask_len - i - 1].re = mn * (i as f32 + 1.0).powf(powf32);
    }
    let mut sum: f32 = 0.0;
    for i in 0..mask_len{
        sum = sum + mask_buffer[i].re;
    }
    for i in 0..mask_len{
        mask_buffer[i].re = mask_buffer[i].re/sum;
        //print!(" mask_buffer[{}] {} ", i, sum);
    }
}