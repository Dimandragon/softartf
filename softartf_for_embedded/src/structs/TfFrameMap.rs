use std::collections::HashMap;
use num_complex::Complex;
use std::ops::{Index, IndexMut};
use crate::structs::FrSqIFCompute::FrSqIFCompute;
use crate::math::resampling::back_resampling;
//use crate::structs::FIFCompute::FIFCompute;
use softartf_enums::InstFreqTypeForResampling;
use softartf_enums::InstAmplType;

pub struct TfFrameMap{
    data: Vec<HashMap<i32, f32>>,
    inst_freq_type: InstFreqTypeForResampling,
    inst_ampl_type: InstAmplType,
}

impl TfFrameMap {
    pub fn new(frsqif_computing: &FrSqIFCompute, time_kvant_samples: Option<usize>, time_kvant_seconds: Option<f32>)->TfFrameMap{
        assert!((time_kvant_samples.is_some()!=time_kvant_seconds.is_some()));
        let mut frame_size: usize = 0;
        if time_kvant_samples.is_some(){
            frame_size = frsqif_computing.signal_len/time_kvant_samples.unwrap();
        }
        if time_kvant_seconds.is_some(){
            frame_size = (frsqif_computing.signal_len as f32/frsqif_computing.sample_freq/time_kvant_seconds.unwrap()) as usize;
        }
        TfFrameMap{
            data: vec![HashMap::new(); frame_size],
            inst_freq_type: InstFreqTypeForResampling::Simple,
            inst_ampl_type: InstAmplType::Simple,
        }
    }
    
    pub fn reconfig(&mut self, frsqif_computing: &FrSqIFCompute, time_kvant_samples: Option<usize>, time_kvant_seconds: Option<f32>){
        assert!((time_kvant_samples.is_some()!=time_kvant_seconds.is_some()));
        let mut frame_size: usize = 0;
        if time_kvant_samples.is_some(){
            frame_size = frsqif_computing.signal_len/time_kvant_samples.unwrap();
        }
        if time_kvant_seconds.is_some(){
            frame_size = (frsqif_computing.signal_len as f32 /frsqif_computing.sample_freq/time_kvant_seconds.unwrap()) as usize;
        }
        if self.data.len()<frame_size{
            for _i in self.data.len()..frame_size{
                self.data.push(HashMap::new());
            }
        }
        if self.data.len()>frame_size{
            self.data.truncate(frame_size);
        }
    }
    pub fn zeroes(&mut self){
        for i in 0..self.data.len(){
            self.data[i].clear();
        }
    }
    pub fn len(&self)->usize{
        self.data.len()
    }
    pub fn add_data(&mut self, frsqif_computing: &mut FrSqIFCompute, freq_detail_hz: Option<f32>, freq_detail_period_sec: Option<f32>, time_kvant_samples: Option<usize>, time_kvant_seconds: Option<f32>, inst_freq: &mut Vec<f32>, inst_ampl: &mut Vec<f32>, imf_buffer: &mut Vec<Complex<f32>>){
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


        match self.inst_freq_type {
            InstFreqTypeForResampling::Simple => {
                crate::math::inst_freq::inst_freq(&mut imf_buffer[0..temp_len]
                                                  ,&mut inst_freq[0..temp_len]
                                                  ,frsqif_computing.sample_freq
                                                  ,frsqif_computing.signal.fft
                                                  ,frsqif_computing.signal.ifft);
            },
            InstFreqTypeForResampling::Extremums => {
                crate::math::inst_freq::inst_freq_extremums_complex(&mut imf_buffer[0..temp_len]
                                                            , &mut inst_freq[0..temp_len]
                                                            , frsqif_computing.sample_freq * (temp_len as f32) / (frsqif_computing.signal_len as f32)
                                                            , &mut frsqif_computing.signal.extremums_vec
                                                            , frsqif_computing.signal.kvant);
            },
            InstFreqTypeForResampling::ExtremumsAveraging => {
                crate::math::inst_freq::inst_freq_extremums_averaging_complex(&mut imf_buffer[0..temp_len]
                                                                      ,&mut inst_freq[0..temp_len]
                                                                      , frsqif_computing.sample_freq * (temp_len as f32) / (frsqif_computing.signal_len as f32)
                                                                      , &mut frsqif_computing.signal.extremums_vec
                                                                      , &mut frsqif_computing.signal.freq_x
                                                                      , &mut frsqif_computing.signal.freq_y
                                                                      , frsqif_computing.signal.kvant);
            },
            InstFreqTypeForResampling::SimpleAveraging => {
                crate::math::inst_freq::inst_freq(&mut imf_buffer[0..temp_len]
                                                  ,&mut inst_freq[0..temp_len]
                                                  ,frsqif_computing.sample_freq
                                                  ,frsqif_computing.signal.fft
                                                  ,frsqif_computing.signal.ifft);

                frsqif_computing.averaging.simple_average_inst_freq(&mut inst_freq[0..temp_len]);
            },
        }


        match self.inst_ampl_type {
            InstAmplType::Simple =>{
                crate::math::inst_freq::inst_ampl(&mut imf_buffer[0..temp_len]
                                                  ,&mut inst_ampl[0..temp_len]
                                                  ,frsqif_computing.signal.fft
                                                  ,frsqif_computing.signal.ifft);
            },
            InstAmplType::Extremums =>{
                crate::math::inst_freq::inst_ampl_extremums_complex(&mut imf_buffer[0..temp_len]
                                                            , &mut inst_ampl[0..temp_len]
                                                            , &mut frsqif_computing.signal.extremums_vec
                                                            , frsqif_computing.signal.kvant);
            },
            InstAmplType::ExtremumsAveraging =>{
                crate::math::inst_freq::inst_ampl_extremums_averaging_complex(&mut imf_buffer[0..temp_len]
                                                                      ,&mut inst_ampl[0..temp_len]
                                                                      , &mut frsqif_computing.signal.extremums_vec
                                                                      , &mut frsqif_computing.signal.freq_x
                                                                      , &mut frsqif_computing.signal.freq_y
                                                                      , frsqif_computing.signal.kvant);
            },
            InstAmplType::SimpleAveraging =>{
                crate::math::inst_freq::inst_ampl (&mut imf_buffer[0..temp_len]
                                                   ,&mut inst_ampl[0..temp_len]
                                                   ,frsqif_computing.signal.fft
                                                   ,frsqif_computing.signal.ifft);

                frsqif_computing.averaging.simple_average_inst_freq(&mut inst_ampl[0..temp_len]);
            }
        }


        let mut vec_re: Vec<f32> = Vec::new();
        let mut vec_im: Vec<f32> = Vec::new();
        let mut vec_ampl: Vec<f32> = Vec::new();
        for i in 0..inst_ampl.len(){
            vec_re.push(imf_buffer[i].re);
            vec_im.push(imf_buffer[i].im);
            vec_ampl.push(inst_ampl[i]);
        }


        if time_kvant_samples.is_some() && freq_detail_hz.is_some(){
            for i in 0..inst_freq.len(){
                let index =  (i / time_kvant_samples.unwrap()) as usize;
                let index2 = (inst_freq[i] / freq_detail_hz.unwrap()) as i32;
                if index<self.data.len(){
                    if self.data[index].get(& index2).is_none(){
                        self.data[index].insert (index2, inst_ampl[i]);
                    }
                    else if self.data[index].get(& index2).is_some(){
                        let temp3 = *self.data[index].get(& index2).unwrap();
                        if let Some(x) = self.data[index].get_mut(&index2){
                            *x = temp3 + inst_ampl[i];
                        }
                    }
                }
            }
        }
        if time_kvant_seconds.is_some() && freq_detail_hz.is_some(){
            for i in 0..inst_freq.len(){
                let index = ( i as f32 /frsqif_computing.sample_freq /time_kvant_seconds.unwrap()) as usize;
                let index2 = (inst_freq[i] / freq_detail_hz.unwrap()) as i32;
                if index<self.data.len(){
                    if self.data[index].get(& index2).is_none(){
                        self.data[index].insert (index2, inst_ampl[i]);
                    }
                    else if self.data[index].get(& index2).is_some(){
                        let temp3 = *self.data[index].get(& index2).unwrap();
                        if let Some(x) = self.data[index].get_mut(&index2){
                            *x = temp3 + inst_ampl[i];
                        }
                    }
                }
            }
        }
        if time_kvant_samples.is_some() && freq_detail_period_sec.is_some(){
            for i in 0..inst_freq.len(){
                let index =  (i / time_kvant_samples.unwrap()) as usize;
                let index2 = (1.0/ inst_freq[i] / freq_detail_period_sec.unwrap()) as i32;
                if index<self.data.len(){
                    if self.data[index].get(& index2).is_none(){
                        self.data[index].insert (index2, inst_ampl[i]);
                    }
                    else if self.data[index].get(& index2).is_some(){
                        let temp3 = *self.data[index].get(& index2).unwrap();
                        if let Some(x) = self.data[index].get_mut(&index2){
                            *x = temp3 + inst_ampl[i];
                        }
                    }
                }
            }
        }
        if time_kvant_seconds.is_some() && freq_detail_period_sec.is_some(){
            for i in 0..inst_freq.len(){
                let index = (i as f32 / frsqif_computing.sample_freq/ time_kvant_seconds.unwrap()) as usize;
                let index2 = (1.0 / inst_freq[i]/ freq_detail_period_sec.unwrap()) as i32;
                if index<self.data.len(){
                    if self.data[index].get(& index2).is_none(){
                        self.data[index].insert (index2, inst_ampl[i]);
                    }
                    else if self.data[index].get(& index2).is_some(){
                        let temp3 = *self.data[index].get(& index2).unwrap();
                        if let Some(x) = self.data[index].get_mut(&index2){
                            *x = temp3 + inst_ampl[i];
                        }
                    }
                }
            }
        }
    }

    pub fn set_inst_freq_type(&mut self, inst_freq_type: InstFreqTypeForResampling){
        self.inst_freq_type = inst_freq_type;
    }
    pub fn set_inst_ampl_type(&mut self, inst_ampl_type: InstAmplType){
        self.inst_ampl_type = inst_ampl_type;
    }

    /*pub fn new_fif(fif_computing: &FIFCompute, time_kvant_samples: Option<usize>, time_kvant_seconds: Option<f32>)->TfFrameMap{
        assert!((time_kvant_samples.is_some()!=time_kvant_seconds.is_some()));
        let mut frame_size: usize = 0;
        if time_kvant_samples.is_some(){
            frame_size = fif_computing.signal_len/time_kvant_samples.unwrap();
        }
        if time_kvant_seconds.is_some(){
            frame_size = (fif_computing.signal_len as f32/fif_computing.sample_freq/time_kvant_seconds.unwrap()) as usize;
        }
        TfFrameMap{
            data: vec![HashMap::new(); frame_size],
            inst_freq_type: InstFreqTypeForResampling::Simple,
            inst_ampl_type: InstAmplType::Simple,
        }
    }


    pub fn reconfig_fif(&mut self, fif_computing: &FIFCompute, time_kvant_samples: Option<usize>, time_kvant_seconds: Option<f32>){
        assert!((time_kvant_samples.is_some()!=time_kvant_seconds.is_some()));
        let mut frame_size: usize = 0;
        if time_kvant_samples.is_some(){
            frame_size = fif_computing.signal_len/time_kvant_samples.unwrap();
        }
        if time_kvant_seconds.is_some(){
            frame_size = (fif_computing.signal_len as f32 /fif_computing.sample_freq/time_kvant_seconds.unwrap()) as usize;
        }
        if self.data.len()<frame_size{
            for _i in self.data.len()..frame_size{
                self.data.push(HashMap::new());
            }
        }
        if self.data.len()>frame_size{
            self.data.truncate(frame_size);
        }
    }
    pub fn add_data_fif(&mut self, fif_computing: &mut FIFCompute, freq_detail_hz: Option<f32>, freq_detail_period_sec: Option<f32>, time_kvant_samples: Option<usize>, time_kvant_seconds: Option<f32>, inst_freq: &mut Vec<f32>, inst_ampl: &mut Vec<f32>, imf_buffer: &mut Vec<Complex<f32>>){
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

        inst_freq_ampl (&mut imf_buffer[0..temp_len],
                        &mut inst_freq[0..temp_len],
                        &mut inst_ampl[0..temp_len],
                        fif_computing.sample_freq,
                        &mut fif_computing.signal.planner);


        if time_kvant_samples.is_some() && freq_detail_hz.is_some(){
            for i in 0..inst_freq.len(){
                let index =  (i / time_kvant_samples.unwrap()) as usize;
                let index2 = (inst_freq[i] / freq_detail_hz.unwrap()) as i32;
                if index<self.data.len(){
                    if self.data[index].get(& index2).is_none(){
                        self.data[index].insert (index2, inst_ampl[i]);
                    }
                    else if self.data[index].get(& index2).is_some(){
                        let temp3 = *self.data[index].get(& index2).unwrap();
                        if let Some(x) = self.data[index].get_mut(&index2){
                            *x = temp3 + inst_ampl[i];
                        }
                    }
                }
            }
        }
        if time_kvant_seconds.is_some() && freq_detail_hz.is_some(){
            for i in 0..inst_freq.len(){
                let index = ( i as f32 /fif_computing.sample_freq /time_kvant_seconds.unwrap()) as usize;
                let index2 = (inst_freq[i] / freq_detail_hz.unwrap()) as i32;
                if index<self.data.len(){
                    if self.data[index].get(& index2).is_none(){
                        self.data[index].insert (index2, inst_ampl[i]);
                    }
                    else if self.data[index].get(& index2).is_some(){
                        let temp3 = *self.data[index].get(& index2).unwrap();
                        if let Some(x) = self.data[index].get_mut(&index2){
                            *x = temp3 + inst_ampl[i];
                        }
                    }
                }
            }
        }
        if time_kvant_samples.is_some() && freq_detail_period_sec.is_some(){
            for i in 0..inst_freq.len(){
                let index =  (i / time_kvant_samples.unwrap()) as usize;
                let index2 = (1.0/ inst_freq[i] / freq_detail_period_sec.unwrap()) as i32;
                if index<self.data.len(){
                    if self.data[index].get(& index2).is_none(){
                        self.data[index].insert (index2, inst_ampl[i]);
                    }
                    else if self.data[index].get(& index2).is_some(){
                        let temp3 = *self.data[index].get(& index2).unwrap();
                        if let Some(x) = self.data[index].get_mut(&index2){
                            *x = temp3 + inst_ampl[i];
                        }
                    }
                }
            }
        }
        if time_kvant_seconds.is_some() && freq_detail_period_sec.is_some(){
            for i in 0..inst_freq.len(){
                let index = (i as f32 / fif_computing.sample_freq/ time_kvant_seconds.unwrap()) as usize;
                let index2 = (1.0 / inst_freq[i]/ freq_detail_period_sec.unwrap()) as i32;
                if index<self.data.len(){
                    if self.data[index].get(& index2).is_none(){
                        self.data[index].insert (index2, inst_ampl[i]);
                    }
                    else if self.data[index].get(& index2).is_some(){
                        let temp3 = *self.data[index].get(& index2).unwrap();
                        if let Some(x) = self.data[index].get_mut(&index2){
                            *x = temp3 + inst_ampl[i];
                        }
                    }
                }
            }
        }
    }*/

}

impl Index<usize> for TfFrameMap{
    type Output = HashMap<i32, f32>;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.data.len());
        &self.data[index]
    }
}
impl IndexMut<usize> for TfFrameMap{
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert! (index<self.len());
        &mut self.data[index]
    }
}