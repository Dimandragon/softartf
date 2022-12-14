use num_complex::Complex;

pub fn my_conv(arr1: &mut [Complex<f32>], arr2: &mut [Complex<f32>], fft: fn(&mut[Complex<f32>]), ifft: fn(&mut[Complex<f32>]),
               filter_len: usize, signal_size: usize, copy_buffer: &mut [Complex<f32>]) {
    assert!(arr1.len() >= signal_size + filter_len / 2);
    assert!(arr2.len() >= signal_size + filter_len / 2);
    assert!(copy_buffer.len() >= filter_len / 2);
    let mut avg: f32 = num_traits::zero();
    for i in 0..signal_size {
        avg = avg + arr1[i].re;
    }
    for i in 0..signal_size{
        arr1[i].im = 0.0;
        arr2[i].im = 0.0;
    }
    avg = avg / (signal_size as f32);
    for i in signal_size..signal_size + filter_len / 2 {
        arr1[i].re = avg;
        arr1[i].im = num_traits::identities::Zero::zero();
    }
    fft_conv(arr1, arr2, fft, ifft, filter_len, copy_buffer);
}
//эта свертка дополняет входной сигнал средними значениями сигнала вместо нулей и выполняет центровку свертки по центру фильтра
pub fn fft_conv (arr1: &mut[Complex<f32>], arr2: &mut [Complex<f32>], fft: fn(&mut[Complex<f32>]), ifft: fn(&mut[Complex<f32>]),
                 filter_len: usize, copy_buffer: &mut [Complex<f32>]) {
    assert!((arr1.len()==arr2.len()));
    assert!(copy_buffer.len()>=filter_len/2);
    for i in 0..arr1.len(){
        arr1[i].im = 0.0;
        arr2[i].im = 0.0;
    }
    //let fft1 = planner.plan_fft_forward(arr1.len());
    //let ifft = planner.plan_fft_inverse(arr1.len());
    fft(arr1);
    fft(arr2);
    for i in 0..arr1.len(){
        arr1[i] = arr1[i]*arr2[i];
    }
    ifft(arr1);
    ifft(arr2);
    for i in 0..filter_len/2{
        copy_buffer[i] = arr1[i];
    }
    for i in 0..arr1.len()-filter_len/2{
        arr1[i] = arr1[i+filter_len/2];
    }
    for i in arr1.len()-filter_len/2..arr1.len(){
        arr1[i] = copy_buffer[i+filter_len/2-arr1.len()];
    }
    for i in 0..arr1.len(){
        arr1[i].re = num_traits::float::Float::sqrt(arr1[i].re*arr1[i].re + arr1[i].im*arr1[i].im);
        arr1[i].im = 0.0;
        arr2[i].re = num_traits::float::Float::sqrt(arr2[i].re*arr2[i].re + arr2[i].im*arr2[i].im);
        arr2[i].im = 0.0;
    }
}
