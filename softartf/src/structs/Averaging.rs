use crate::structs::CycleBuffer::CycleBuffer;
use crate::structs::FrSqIFCompute::FrSqIFCompute;
use crate::math::instrumental_math::sqr_avg_diff;
use crate::math::instrumental_math::averaging;


pub struct Averaging{
    average_buffer: CycleBuffer<f32>,
    averaging_window_len: usize,
    run: bool,
    freq_desp_averaging: bool,
    freq_desp_averaging_len_mn: f32,
}

impl Averaging{
    pub fn new()-> Averaging{
        Averaging{
            average_buffer: CycleBuffer::new_with_len(0),
            averaging_window_len: 0,
            run: false,
            freq_desp_averaging: false,
            freq_desp_averaging_len_mn: 0.0,
        }
    }
    pub fn set_averaging_window_len(&mut self, new_averaging_window_len: usize){
        self.freq_desp_averaging = false;
        self.averaging_window_len = new_averaging_window_len;
        self.average_buffer.resize(new_averaging_window_len);
    }
    pub fn set_freq_desp_averaging_len_mn(&mut self, new_freq_desp_averaging_len_mn: f32){
        self.freq_desp_averaging_len_mn = new_freq_desp_averaging_len_mn;
    }
    pub fn set_run(&mut self, new_run: bool){
        self.run = new_run;
    }
    pub fn set_freq_desp_averaging(&mut self, new_freq_desp_averaging: bool){
        self.freq_desp_averaging = new_freq_desp_averaging;
    }
    pub fn average(&mut self, arr: &mut [f32], frsqif: &FrSqIFCompute, inst_freq: &[f32]){
        if self.run == true{
            let temp_len = inst_freq.len();
            if self.freq_desp_averaging == true{
                let mut sum: f32 = 0.0;
                let mut min_freq_temp: f32 = std::f32::MAX;
                let mut max_freq_temp: f32 = 0.0;
                for i in 0..arr.len(){
                    sum = sum + inst_freq[i] * frsqif.new_freq_conv[i];
                    if min_freq_temp>inst_freq[i]{
                        min_freq_temp = inst_freq[i];
                    }
                    if max_freq_temp<inst_freq[i]{
                        max_freq_temp = inst_freq[i];
                    }
                }
                let sq_avg_freq_diff = 1.0 / sqr_avg_diff(&inst_freq[1..inst_freq.len()]);
                let avg_window_len = sq_avg_freq_diff * self.freq_desp_averaging_len_mn * (max_freq_temp-min_freq_temp);
                if (avg_window_len as usize)>arr.len(){
                    averaging(&mut arr[0..temp_len], temp_len, &mut self.average_buffer);
                }
                else{
                    averaging(&mut arr[0..temp_len], avg_window_len as usize, &mut self.average_buffer);
                }
            }
            else{
                let mut avg_window_len = self.averaging_window_len;
                if avg_window_len > temp_len{
                    avg_window_len = temp_len;
                }
                averaging(&mut arr[0..temp_len], avg_window_len, &mut self.average_buffer);
            }
        }
    }

    pub fn average_inst_freq(&mut self, inst_freq: &mut [f32], new_freq_conv: &mut [f32]){
        if self.run == true{
            let temp_len = inst_freq.len();
            if self.freq_desp_averaging == true{
                let mut sum: f32 = 0.0;
                let mut min_freq_temp: f32 = std::f32::MAX;
                let mut max_freq_temp: f32 = 0.0;
                for i in 0..inst_freq.len(){
                    sum = sum + inst_freq[i] * new_freq_conv[i];
                    if min_freq_temp>inst_freq[i]{
                        min_freq_temp = inst_freq[i];
                    }
                    if max_freq_temp<inst_freq[i]{
                        max_freq_temp = inst_freq[i];
                    }
                }
                let sq_avg_freq_diff = 1.0 / crate::math::instrumental_math::sqr_avg_diff(&inst_freq[1..inst_freq.len()]);
                let avg_window_len = sq_avg_freq_diff * self.freq_desp_averaging_len_mn * (max_freq_temp-min_freq_temp);
                if (avg_window_len as usize)>inst_freq.len(){
                    averaging(&mut inst_freq[0..temp_len], temp_len, &mut self.average_buffer);
                }
                else{
                    averaging(&mut inst_freq[0..temp_len], avg_window_len as usize, &mut self.average_buffer);
                }
            }
            else{
                let mut avg_window_len = self.averaging_window_len;
                if avg_window_len > temp_len{
                    avg_window_len = temp_len;
                }
                averaging(&mut inst_freq[0..temp_len], avg_window_len, &mut self.average_buffer);
            }
        }
    }

    pub fn simple_average_inst_freq(&mut self, inst_freq: &mut [f32]){
        if self.run == true{
            let temp_len = inst_freq.len();
            if self.freq_desp_averaging == true{
                let mut sum: f32 = 0.0;
                let mut min_freq_temp: f32 = std::f32::MAX;
                let mut max_freq_temp: f32 = 0.0;
                for i in 0..inst_freq.len(){
                    sum = sum + inst_freq[i];
                    if min_freq_temp>inst_freq[i]{
                        min_freq_temp = inst_freq[i];
                    }
                    if max_freq_temp<inst_freq[i]{
                        max_freq_temp = inst_freq[i];
                    }
                }
                let sq_avg_freq_diff = 1.0 / crate::math::instrumental_math::sqr_avg_diff(&inst_freq[1..inst_freq.len()]);
                let avg_window_len = sq_avg_freq_diff * self.freq_desp_averaging_len_mn * (max_freq_temp-min_freq_temp);
                if (avg_window_len as usize)>inst_freq.len(){
                    averaging(&mut inst_freq[0..temp_len], temp_len, &mut self.average_buffer);
                }
                else{
                    averaging(&mut inst_freq[0..temp_len], avg_window_len as usize, &mut self.average_buffer);
                }
            }
            else{
                let mut avg_window_len = self.averaging_window_len;
                if avg_window_len > temp_len{
                    avg_window_len = temp_len;
                }
                averaging(&mut inst_freq[0..temp_len], avg_window_len, &mut self.average_buffer);
            }
        }
    }
}