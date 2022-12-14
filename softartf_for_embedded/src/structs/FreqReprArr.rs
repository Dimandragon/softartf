use num_complex::Complex;
use std::ops::{Index, IndexMut};
//use crate::{FIF, FIFCompute};
use crate::structs::FrSqIFCompute::FrSqIFCompute;
use crate::math::resampling::back_resampling;


pub struct FreqReprArr{
    data: Vec<f32>,
}

impl FreqReprArr{
    pub fn len(&self)->usize{
        self.data.len()
    }
    pub fn new(frsqif_computing: &FrSqIFCompute, freq_detail_hz: Option<f32>, freq_detail_period_sec: Option<f32>)->FreqReprArr{
        assert!((freq_detail_hz.is_some()!=freq_detail_period_sec.is_some()));
        let mut new_freq_repr_len: usize = 0;
        if freq_detail_hz.is_some(){
            new_freq_repr_len = (frsqif_computing.sample_freq/2.0/freq_detail_hz.unwrap()) as usize;
            if new_freq_repr_len > frsqif_computing.signal_len/2{
                new_freq_repr_len = frsqif_computing.signal_len/2;
            }
        }
        if freq_detail_period_sec.is_some(){
            new_freq_repr_len = (frsqif_computing.signal_len as f32 /frsqif_computing.sample_freq/freq_detail_period_sec.unwrap()) as usize;
        }
        FreqReprArr{
            data: vec![0.0; new_freq_repr_len],
        }
    }
    pub fn reconfig(&mut self, frsqif_computing: &FrSqIFCompute, freq_detail_hz: Option<f32>, freq_detail_period_sec: Option<f32>){
        assert!((freq_detail_hz.is_some()!=freq_detail_period_sec.is_some()));
        let mut new_freq_repr_len: usize = 0;
        if freq_detail_hz.is_some(){
            new_freq_repr_len = (frsqif_computing.sample_freq/2.0/freq_detail_hz.unwrap()) as usize;
            if new_freq_repr_len > frsqif_computing.signal_len/2{
                new_freq_repr_len = frsqif_computing.signal_len/2;
            }
        }
        if freq_detail_period_sec.is_some(){
            new_freq_repr_len = (frsqif_computing.signal_len as f32/frsqif_computing.sample_freq/freq_detail_period_sec.unwrap()) as usize;
        }
        if self.len()>new_freq_repr_len{
            self.data.truncate(new_freq_repr_len);
        }
        if self.len()<new_freq_repr_len{
            for _i in self.len()..new_freq_repr_len{
                self.data.push(0.0);
            }
        }
        self.zeroes();
    }
    pub fn zeroes(&mut self){
        for i in 0..self.data.len(){
            self.data[i]=0.0;
        }
    }
    pub fn add_data(&mut self, frsqif_computing: &mut FrSqIFCompute, freq_detail_hz: Option<f32>, freq_detail_period_sec: Option<f32>, inst_freq: &mut Vec<f32>, inst_ampl: &mut Vec<f32>, imf_buffer: &mut Vec<Complex<f32>>){
        if frsqif_computing.continue_status{
            back_resampling(& frsqif_computing.signal_image[0..frsqif_computing.contains_len],
                            imf_buffer,
                            &mut frsqif_computing.new_freq_conv,
                            frsqif_computing.signal_len);
        }
        else{
            back_resampling(& frsqif_computing.signal.get_slice(),
                            imf_buffer,
                            &mut frsqif_computing.new_freq_conv,
                            frsqif_computing.signal_len);
        }

        let temp_len = imf_buffer.len();

        if inst_freq.len()<temp_len{
            for _i in inst_freq.len()..imf_buffer.len(){
                inst_freq.push(0.0);
            }
        }
        if inst_freq.len()>temp_len{
            inst_freq.truncate(temp_len);
        }
        if inst_ampl.len()<temp_len{
            for _i in inst_ampl.len()..imf_buffer.len(){
                inst_ampl.push(0.0);
            }
        }
        if inst_ampl.len()>temp_len{
            inst_ampl.truncate(temp_len);
        }

        crate::math::inst_freq::inst_ampl(&mut imf_buffer[0..temp_len]
                                          ,&mut inst_ampl[0..temp_len]
                                          ,frsqif_computing.signal.fft
                                          ,frsqif_computing.signal.ifft);

        crate::math::inst_freq::inst_freq_extremums_averaging_complex(&mut imf_buffer[0..temp_len]
                                                                      , &mut inst_freq[0..temp_len]
                                                                      , frsqif_computing.sample_freq
                                                                      , &mut frsqif_computing.signal.extremums_vec
                                                                      , &mut frsqif_computing.signal.freq_x
                                                                      , &mut frsqif_computing.signal.freq_y
                                                                      , frsqif_computing.signal.kvant);


        if freq_detail_hz.is_some(){
            for i in 0..inst_freq.len(){
                let index = (inst_freq[i] / freq_detail_hz.unwrap() * frsqif_computing.signal_len as f32 / inst_freq.len() as f32) as usize;
                if index<self.data.len(){
                    let temp = self[index];
                    self[index] = temp + inst_ampl[i];// /(frsqif_computing.signal_len as f32);
                }
            }
        }

        if freq_detail_period_sec.is_some(){
            for i in 0..inst_freq.len(){
                let index = ( 1.0 / inst_freq[i] / freq_detail_period_sec.unwrap() * frsqif_computing.signal_len as f32 / inst_freq.len() as f32) as usize;
                if index<self.data.len(){
                    let temp = self[index];
                    self[index] = temp + inst_ampl[i];// /(frsqif_computing.signal_len as f32);
                }
            }
        }
    }


    /*pub fn new_fif(fif_computing: &FIFCompute, freq_detail_hz: Option<f32>, freq_detail_period_sec: Option<f32>)->FreqReprArr{
        assert!((freq_detail_hz.is_some()!=freq_detail_period_sec.is_some()));
        let mut new_freq_repr_len: usize = 0;
        if freq_detail_hz.is_some(){
            new_freq_repr_len = (fif_computing.sample_freq/2.0/freq_detail_hz.unwrap()) as usize;
            if new_freq_repr_len > fif_computing.signal_len/2{
                new_freq_repr_len = fif_computing.signal_len/2;
            }
        }
        if freq_detail_period_sec.is_some(){
            new_freq_repr_len = (fif_computing.signal_len as f32 /fif_computing.sample_freq/freq_detail_period_sec.unwrap()) as usize;
        }
        FreqReprArr{
            data: vec![0.0; new_freq_repr_len],
        }
    }
    pub fn reconfig_fif(&mut self, fif_computing: &FIFCompute, freq_detail_hz: Option<f32>, freq_detail_period_sec: Option<f32>){
        assert!((freq_detail_hz.is_some()!=freq_detail_period_sec.is_some()));
        let mut new_freq_repr_len: usize = 0;
        if freq_detail_hz.is_some(){
            new_freq_repr_len = (fif_computing.sample_freq/2.0/freq_detail_hz.unwrap()) as usize;
            if new_freq_repr_len > fif_computing.signal_len/2{
                new_freq_repr_len = fif_computing.signal_len/2;
            }
        }
        if freq_detail_period_sec.is_some(){
            new_freq_repr_len = (fif_computing.signal_len as f32/fif_computing.sample_freq/freq_detail_period_sec.unwrap()) as usize;
        }
        if self.len()>new_freq_repr_len{
            self.data.truncate(new_freq_repr_len);
        }
        if self.len()<new_freq_repr_len{
            for _i in self.len()..new_freq_repr_len{
                self.data.push(0.0);
            }
        }
        self.zeroes();
    }
    pub fn add_data_fif(&mut self, fif_computing: &mut FIFCompute, freq_detail_hz: Option<f32>, freq_detail_period_sec: Option<f32>, inst_freq: &mut Vec<f32>, inst_ampl: &mut Vec<f32>, imf_buffer: &mut Vec<Complex<f32>>){
        if fif_computing.continue_status{
            imf_buffer.clear();
            for i in 0..fif_computing.signal_image.len(){
                imf_buffer.push(fif_computing.signal_image[i]);
            }
        }
        else{
            imf_buffer.clear();
            for i in 0..fif_computing.signal.len(){
                imf_buffer.push(fif_computing.signal[i]);
            }
        }

        let temp_len = imf_buffer.len();

        if inst_freq.len()<temp_len{
            for _i in inst_freq.len()..imf_buffer.len(){
                inst_freq.push(0.0);
            }
        }
        if inst_freq.len()>temp_len{
            inst_freq.truncate(temp_len);
        }
        if inst_ampl.len()<temp_len{
            for _i in inst_ampl.len()..imf_buffer.len(){
                inst_ampl.push(0.0);
            }
        }
        if inst_ampl.len()>temp_len{
            inst_ampl.truncate(temp_len);
        }

        crate::math::inst_freq::inst_ampl(&mut imf_buffer[0..temp_len],
                                          &mut inst_ampl[0..temp_len],
                                          &mut fif_computing.signal.planner);

        crate::math::inst_freq::inst_freq_extremums_averaging_complex(&mut imf_buffer[0..temp_len]
                                                                      , &mut inst_freq[0..temp_len]
                                                                      , fif_computing.sample_freq
                                                                      , &mut fif_computing.signal.extremums_vec
                                                                      , &mut fif_computing.signal.freq_x
                                                                      , &mut fif_computing.signal.freq_y
                                                                      , fif_computing.signal.kvant);


        if freq_detail_hz.is_some(){
            for i in 0..inst_freq.len(){
                let index = (inst_freq[i] / freq_detail_hz.unwrap() * fif_computing.signal_len as f32 / inst_freq.len() as f32) as usize;
                if index<self.data.len(){
                    let temp = self[index];
                    self[index] = temp + inst_ampl[i];// /(frsqif_computing.signal_len as f32);
                }
            }
        }

        if freq_detail_period_sec.is_some(){
            for i in 0..inst_freq.len(){
                let index = ( 1.0 / inst_freq[i] / freq_detail_period_sec.unwrap() * fif_computing.signal_len as f32 / inst_freq.len() as f32) as usize;
                if index<self.data.len(){
                    let temp = self[index];
                    self[index] = temp + inst_ampl[i];// /(frsqif_computing.signal_len as f32);
                }
            }
        }
    }*/
}

impl Index<usize> for FreqReprArr{
    type Output = f32;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.data.len());
        &self.data[index]
    }
}
impl IndexMut<usize> for FreqReprArr{
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert! (index<self.data.len());
        &mut self.data[index]
    }
}