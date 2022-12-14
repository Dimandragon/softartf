use num_complex::Complex;
use std::ops::{Index, IndexMut};
use crate::structs::FrSqIFCompute::FrSqIFCompute;
use crate::math::resampling::back_resampling;
use crate::math::inst_freq::inst_freq_ampl;
//use crate::structs::FIFCompute::FIFCompute;

pub struct TfFrameArr{
    data: Vec<Vec<f32>>,
}

impl TfFrameArr{
    pub fn new(frsqif_computing: &FrSqIFCompute, freq_detail_hz: Option<f32>, freq_detail_period_sec: Option<f32>, time_kvant_samples: Option<usize>, time_kvant_seconds: Option<f32>)->TfFrameArr{
        assert!((time_kvant_samples.is_some()!=time_kvant_seconds.is_some()));
        let mut frame_size: usize = 0;
        if time_kvant_samples.is_some(){
            frame_size = frsqif_computing.signal_len/time_kvant_samples.unwrap();
        }
        if time_kvant_seconds.is_some(){
            frame_size = (frsqif_computing.signal_len as f32/frsqif_computing.sample_freq /time_kvant_seconds.unwrap()) as usize;
        }
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
        TfFrameArr{
            data: vec![vec![0.0; new_freq_repr_len]; frame_size],
        }
    }
    pub fn reconfig(&mut self, frsqif_computing: &FrSqIFCompute, freq_detail_hz: Option<f32>, freq_detail_period_sec: Option<f32>, time_kvant_samples: Option<usize>, time_kvant_seconds: Option<f32>){
        assert!((time_kvant_samples.is_some()!=time_kvant_seconds.is_some()));
        let mut frame_size: usize = 0;
        if time_kvant_samples.is_some(){
            frame_size = frsqif_computing.signal_len/time_kvant_samples.unwrap();
        }
        if time_kvant_seconds.is_some(){
            frame_size = (frsqif_computing.signal_len as f32 /frsqif_computing.sample_freq /time_kvant_seconds.unwrap()) as usize;
        }
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
        if self.data.len()<frame_size{
            for _i in self.data.len()..frame_size{
                self.data.push(vec![0.0; new_freq_repr_len]);
            }
        }
        if self.data.len()>frame_size{
            self.data.truncate(frame_size);
        }
        for i in 0..self.data.len(){
            if self.data[i].len()>new_freq_repr_len{
                self.data[i].truncate(new_freq_repr_len);
            }
            if self.data[i].len()<new_freq_repr_len{
                for _j in self.data[i].len()..new_freq_repr_len{
                    self.data[i].push(0.0);
                }
            }
        }
        self.zeroes();
    }
    pub fn zeroes(&mut self){
        for i in 0..self.data.len(){
            for j in 0..self.data[i].len(){
                self.data[i][j] = 0.0;
            }
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

        inst_freq_ampl (&mut imf_buffer[0..temp_len]
                        , &mut inst_freq[0..temp_len]
                        , &mut inst_ampl[0..temp_len]
                        , frsqif_computing.sample_freq
                        , frsqif_computing.signal.fft
                        , frsqif_computing.signal.ifft);

        if time_kvant_samples.is_some() && freq_detail_hz.is_some(){
            for i in 0..inst_freq.len(){
                let index =  i / time_kvant_samples.unwrap();
                let index2 = (inst_freq[i] / freq_detail_hz.unwrap()) as usize;
                if index<self.data.len(){
                    let temp =  self.data[index][index2];
                    self.data[index][index2] = temp + inst_ampl[i]/(frsqif_computing.signal_len as f32);
                }
            }
        }
        if time_kvant_seconds.is_some() && freq_detail_hz.is_some(){
            for i in 0..inst_freq.len(){
                let index = (i as f32 /frsqif_computing.sample_freq /time_kvant_seconds.unwrap()) as usize;
                let index2 = (inst_freq[i] / freq_detail_hz.unwrap()) as usize;
                if index<self.data.len(){
                    let temp =  self.data[index][index2];
                    self.data[index][index2] = temp + inst_ampl[i]/(frsqif_computing.signal_len as f32);
                }
            }
        }
        if time_kvant_samples.is_some() && freq_detail_period_sec.is_some(){
            for i in 0..inst_freq.len(){
                let index =  i / time_kvant_samples.unwrap();
                let index2 = (1.0/ (inst_freq[i] / freq_detail_period_sec.unwrap())) as usize;
                if index<self.data.len(){
                    let temp =  self.data[index][index2];
                    self.data[index][index2] = temp + inst_ampl[i]/(frsqif_computing.signal_len as f32);
                }
            }
        }
        if time_kvant_seconds.is_some() && freq_detail_period_sec.is_some(){
            for i in 0..inst_freq.len(){
                let index = (i as f32 / frsqif_computing.sample_freq/ time_kvant_seconds.unwrap()) as usize;
                let index2 = (1.0 / (inst_freq[i]/ freq_detail_period_sec.unwrap())) as usize;
                if index<self.data.len(){
                    let temp =  self.data[index][index2];
                    self.data[index][index2] = temp + inst_ampl[i]/(frsqif_computing.signal_len as f32);
                }
            }
        }
    }

    /*pub fn new_fif(fif_computing: &FIFCompute, freq_detail_hz: Option<f32>, freq_detail_period_sec: Option<f32>, time_kvant_samples: Option<usize>, time_kvant_seconds: Option<f32>)->TfFrameArr{
        assert!((time_kvant_samples.is_some()!=time_kvant_seconds.is_some()));
        let mut frame_size: usize = 0;
        if time_kvant_samples.is_some(){
            frame_size = fif_computing.signal_len/time_kvant_samples.unwrap();
        }
        if time_kvant_seconds.is_some(){
            frame_size = (fif_computing.signal_len as f32/fif_computing.sample_freq /time_kvant_seconds.unwrap()) as usize;
        }
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
        TfFrameArr{
            data: vec![vec![0.0; new_freq_repr_len]; frame_size],
        }
    }
    pub fn reconfig_fif(&mut self, fif_computing: &FIFCompute, freq_detail_hz: Option<f32>, freq_detail_period_sec: Option<f32>, time_kvant_samples: Option<usize>, time_kvant_seconds: Option<f32>){
        assert!((time_kvant_samples.is_some()!=time_kvant_seconds.is_some()));
        let mut frame_size: usize = 0;
        if time_kvant_samples.is_some(){
            frame_size = fif_computing.signal_len/time_kvant_samples.unwrap();
        }
        if time_kvant_seconds.is_some(){
            frame_size = (fif_computing.signal_len as f32 /fif_computing.sample_freq /time_kvant_seconds.unwrap()) as usize;
        }
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
        if self.data.len()<frame_size{
            for _i in self.data.len()..frame_size{
                self.data.push(vec![0.0; new_freq_repr_len]);
            }
        }
        if self.data.len()>frame_size{
            self.data.truncate(frame_size);
        }
        for i in 0..self.data.len(){
            if self.data[i].len()>new_freq_repr_len{
                self.data[i].truncate(new_freq_repr_len);
            }
            if self.data[i].len()<new_freq_repr_len{
                for _j in self.data[i].len()..new_freq_repr_len{
                    self.data[i].push(0.0);
                }
            }
        }
        self.zeroes();
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
                let index =  i / time_kvant_samples.unwrap();
                let index2 = (inst_freq[i] / freq_detail_hz.unwrap()) as usize;
                if index<self.data.len(){
                    let temp =  self.data[index][index2];
                    self.data[index][index2] = temp + inst_ampl[i]/(fif_computing.signal_len as f32);
                }
            }
        }
        if time_kvant_seconds.is_some() && freq_detail_hz.is_some(){
            for i in 0..inst_freq.len(){
                let index = (i as f32 /fif_computing.sample_freq /time_kvant_seconds.unwrap()) as usize;
                let index2 = (inst_freq[i] / freq_detail_hz.unwrap()) as usize;
                if index<self.data.len(){
                    let temp =  self.data[index][index2];
                    self.data[index][index2] = temp + inst_ampl[i]/(fif_computing.signal_len as f32);
                }
            }
        }
        if time_kvant_samples.is_some() && freq_detail_period_sec.is_some(){
            for i in 0..inst_freq.len(){
                let index =  i / time_kvant_samples.unwrap();
                let index2 = (1.0/ (inst_freq[i] / freq_detail_period_sec.unwrap())) as usize;
                if index<self.data.len(){
                    let temp =  self.data[index][index2];
                    self.data[index][index2] = temp + inst_ampl[i]/(fif_computing.signal_len as f32);
                }
            }
        }
        if time_kvant_seconds.is_some() && freq_detail_period_sec.is_some(){
            for i in 0..inst_freq.len(){
                let index = (i as f32 / fif_computing.sample_freq/ time_kvant_seconds.unwrap()) as usize;
                let index2 = (1.0 / (inst_freq[i]/ freq_detail_period_sec.unwrap())) as usize;
                if index<self.data.len(){
                    let temp =  self.data[index][index2];
                    self.data[index][index2] = temp + inst_ampl[i]/(fif_computing.signal_len as f32);
                }
            }
        }
    }*/

}
impl Index<usize> for TfFrameArr{
    type Output = Vec<f32>;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.data.len());
        &self.data[index]
    }
}
impl IndexMut<usize> for TfFrameArr{
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert! (index<self.len());
        &mut self.data[index]
    }
}