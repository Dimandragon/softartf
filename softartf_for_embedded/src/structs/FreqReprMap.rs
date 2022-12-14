use std::collections::HashMap;
use num_complex::Complex;
use crate::structs::FrSqIFCompute::FrSqIFCompute;
use crate::math::resampling::back_resampling;

pub struct FreqReprMap{
    pub data: HashMap<i32, f32>,
}

impl FreqReprMap{
    pub fn new()->FreqReprMap{
        FreqReprMap{
            data: HashMap::new(),
        }
    }
    pub fn zeroes(&mut self){
        self.data.clear();
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

        /*inst_freq_ampl(&mut imf_buffer[0..temp_len],
                       &mut inst_freq[0..temp_len],
                       &mut inst_ampl[0..temp_len],
                       frsqif_computing.sample_freq,
                       &mut frsqif_computing.signal.planner);*/

        crate::math::inst_freq::inst_ampl(&mut imf_buffer[0..temp_len]
                                          ,&mut inst_ampl[0..temp_len]
                                          , frsqif_computing.signal.fft
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
                let index = (inst_freq[i] / (freq_detail_hz.unwrap() * frsqif_computing.signal_len as f32 / inst_freq.len() as f32)) as i32;

                if self.data.get(& index).is_none(){
                    self.data.insert (index, inst_ampl[i]); // /(frsqif_computing.signal_len as f32));
                }
                else if self.data.get(& index).is_some(){
                    let temp3 = *self.data.get(& index).unwrap();
                    *self.data.get_mut(&index).unwrap() = temp3 + inst_ampl[i]; // /(frsqif_computing.signal_len as f32);
                }
            }
        }

        if freq_detail_period_sec.is_some(){
            for i in 0..inst_freq.len(){
                let index = ( 1.0 / inst_freq[i] / freq_detail_period_sec.unwrap() * frsqif_computing.signal_len as f32 / inst_freq.len() as f32) as i32;

                if self.data.get(& index).is_none(){
                    self.data.insert (index, inst_ampl[i]); // /(frsqif_computing.signal_len as f32));
                }
                else if self.data.get(& index).is_some(){
                    let temp3 = *self.data.get(& index).unwrap();
                    *self.data.get_mut(&index).unwrap() = temp3 + inst_ampl[i]; // /(frsqif_computing.signal_len as f32);
                }
            }
        }
    }


    /*pub fn add_data_fif(&mut self, fif_computing: &mut crate::structs::FIFCompute::FIFCompute, freq_detail_hz: Option<f32>, freq_detail_period_sec: Option<f32>, inst_freq: &mut Vec<f32>, inst_ampl: &mut Vec<f32>, imf_buffer: &mut Vec<Complex<f32>>){
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
                let index = (inst_freq[i] / (freq_detail_hz.unwrap() * fif_computing.signal_len as f32 / inst_freq.len() as f32)) as i32;

                if self.data.get(& index).is_none(){
                    self.data.insert (index, inst_ampl[i]); // /(frsqif_computing.signal_len as f32));
                }
                else if self.data.get(& index).is_some(){
                    let temp3 = *self.data.get(& index).unwrap();
                    *self.data.get_mut(&index).unwrap() = temp3 + inst_ampl[i]; // /(frsqif_computing.signal_len as f32);
                }
            }
        }

        if freq_detail_period_sec.is_some(){
            for i in 0..inst_freq.len(){
                let index = ( 1.0 / inst_freq[i] / freq_detail_period_sec.unwrap() * fif_computing.signal_len as f32 / inst_freq.len() as f32) as i32;

                if self.data.get(& index).is_none(){
                    self.data.insert (index, inst_ampl[i]); // /(frsqif_computing.signal_len as f32));
                }
                else if self.data.get(& index).is_some(){
                    let temp3 = *self.data.get(& index).unwrap();
                    *self.data.get_mut(&index).unwrap() = temp3 + inst_ampl[i]; // /(frsqif_computing.signal_len as f32);
                }
            }
        }
    }*/

}