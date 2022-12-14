pub mod rustff_fft {
    use once_cell::sync::OnceCell;
    use rustfft::FftPlanner;
    use num_complex::Complex;

    //static mut FFT_PLANNER_RUSTFFT: OnceCell<Mutex<FftPlanner<f32>>> = OnceCell::with_value(Mutex::new(FftPlanner::new()));
    static mut FFT_PLANNER_RUSTFFT: OnceCell<FftPlanner<f32>> = OnceCell::new();

    pub fn rustfft_fft(arr: &mut [Complex<f32>]) {
        unsafe {
            if FFT_PLANNER_RUSTFFT.get_mut().as_mut().is_none(){
                FFT_PLANNER_RUSTFFT.set(FftPlanner::new());
            }
            //let fft = FFT_PLANNER_RUSTFFT.get_or_init(||->Mutex<FftPlanner<f32>>{Mutex::new(FftPlanner::new())}).get_mut().unwrap().plan_fft_forward(arr.len());
            let fft = FFT_PLANNER_RUSTFFT.get_mut().as_mut().unwrap().plan_fft_forward(arr.len());
            fft.process(arr);
        }

    }

    pub fn rustfft_ifft(arr: &mut [Complex<f32>]) {
        unsafe {
            if FFT_PLANNER_RUSTFFT.get_mut().as_mut().is_none(){
                FFT_PLANNER_RUSTFFT.set(FftPlanner::new());
            }
            let ifft = FFT_PLANNER_RUSTFFT.get_mut().as_mut().unwrap().plan_fft_inverse(arr.len());
            
            let temp = arr.len() as f32;
            
            for i in 0..arr.len(){
                arr[i].re = arr[i].re / temp;
                arr[i].im = arr[i].im / temp;
            }

            ifft.process(arr);
            
        }
    }
}



//static