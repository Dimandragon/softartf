use num_complex::Complex;
use std::ops::{Index, IndexMut};
use crate::math::inst_freq::freq_normalisation;


pub struct PaddedSignalComplex{
    data: Vec<Complex<f32>>,
    pub len: usize,
    pub fft: fn(&mut[Complex<f32>]),
    pub ifft: fn(&mut[Complex<f32>]),
    copy_buffer: Vec<f32>,
    pub extremums_vec: Vec<usize>,
    pub freq_x: Vec<f32>,
    pub freq_y: Vec<f32>,
    pub kvant: f32,
}

impl Index <usize> for PaddedSignalComplex{
    type Output = Complex<f32>;
    #[inline]
    fn index(&self, index: usize) -> &Complex<f32>{
        assert!(index<self.len);
        &self.data[index+self.len/2]
    }
}
impl IndexMut<usize> for PaddedSignalComplex{
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Complex<f32> {
        assert! (index<self.len);
        &mut self.data[index+self.len/2]
    }
}

impl PaddedSignalComplex{
    pub fn new(fft_in: fn(&mut[Complex<f32>]), ifft_in: fn(&mut[Complex<f32>]))->PaddedSignalComplex{
        PaddedSignalComplex{
            data: Vec::new(),
            len: 0,
            fft: fft_in,
            ifft: ifft_in,
            copy_buffer: Vec::new(),
            extremums_vec: Vec::new(),
            freq_x: Vec::new(),
            freq_y: Vec::new(),
            kvant: 0.1,
        }
    }
    pub fn new_with_len(new_len: usize, fft_in: fn(&mut[Complex<f32>]), ifft_in: fn(&mut[Complex<f32>]))->PaddedSignalComplex{
        PaddedSignalComplex{
            data: vec![Complex{re: 0.0, im: 0.0}; new_len*2],
            len: new_len,
            fft: fft_in,
            ifft: ifft_in,
            copy_buffer: vec![0.0; new_len/2],
            extremums_vec: Vec::new(),
            freq_x: Vec::new(),
            freq_y: Vec::new(),
            kvant: 0.1,
        }
    }
    pub fn set_kvant(&mut self, kvant: f32){
        self.kvant = kvant;
    }
    pub fn resize(&mut self, new_len: usize){
        self.len = new_len;
        self.data.resize(new_len*2, Complex{re: 0.0, im: 0.0});
        self.copy_buffer.resize(new_len/2, 0.0);
    }
    pub fn set_signal(&mut self, data: &Vec<Complex<f32>>){
        self.resize(data.len());
        for i in 0..self.len{
            self[i] = data[i];
        }
    }
    pub fn insert(&mut self, index: usize, value: Complex<f32>){
        assert!(index<self.len);
        self[index] = value;
    }
    pub fn insert_slice(&mut self, index: usize, data: &[Complex<f32>]){
        assert!(index+data.len()<self.data.len());
        for i in 0..data.len(){
            self.insert(index+i, data[i]);
        }
    }
    pub fn padding(&mut self){
        for i in 0..self.len/2{
            self.data[i] = self[self.len/2-i];
        }
        for i in 0..self.len/2{
            self.data[i+self.len/2+self.len] = self.data[self.len/2+self.len-i];
        }
    }
    pub fn get_slice(&self)->&[Complex<f32>]{
        unsafe {
            std::slice::from_raw_parts(&self.data[self.len/2] as *const Complex<f32>, self.len)
        }
    }
    pub fn get_mut_slice(&mut self)->&mut[Complex<f32>]{
        unsafe {
            std::slice::from_raw_parts_mut(&mut self.data[self.len/2] as *mut Complex<f32>, self.len)
        }
    }
    pub fn conv(&mut self, filter: &mut Vec<Complex<f32>>, signal_len: usize, filter_len: usize){
        assert!(signal_len>=filter_len);
        self.padding();
        filter.resize(signal_len*2, Complex{re: 0.0, im: 0.0});
        for i in 0..self.len*2{
            self.data[i].im = 0.0;
            filter[i].im = 0.0;
        }

        (self.fft)(self.data.as_mut_slice());
        (self.fft)(filter.as_mut_slice());

        for i in 0..self.len*2{
            self.data[i] = self.data[i]*filter[i];
        }
        (self.ifft)(self.data.as_mut_slice());
        (self.ifft)(filter.as_mut_slice());


        for i in 0..self.len*2{
            self.data[i].im = 0.0;
            filter[i].im = 0.0;
        }

        for i in self.len/2..self.len+self.len/2{
            self.data[i] = self.data[i+filter_len/2];
        }

        /*assert!(signal_len>=filter_len);
        self.padding();
        filter.resize(signal_len*2, Complex{re: 0.0, im: 0.0});
        for i in 0..self.len*2{
            self.data[i].im = 0.0;
            filter[i].im = 0.0;
        }
    
        (self.fft)(&mut self.data);
        (self.fft)(filter);
        
        for i in 0..self.len*2{
            self.data[i] = self.data[i]*filter[i];
            self.data[i].re = self.data[i].re/(self.data.len() as f32);
            self.data[i].im = self.data[i].im/(self.data.len() as f32);
            filter[i].re = filter[i].re/(self.data.len() as f32);
            filter[i].im = filter[i].im/(self.data.len() as f32);
        }
        
        (self.ifft)(&mut self.data);
        (self.ifft)(filter);
        for i in 0..self.len*2{
            self.data[i].im = 0.0;
            filter[i].im = 0.0;
        }

        for i in self.len/2..self.len+self.len/2{
            self.data[i] = self.data[i+filter_len/2];
        }*/
    }
    pub fn ht(&mut self){
        self.padding();
        for i in 0..self.data.len(){
            self.data[i].im = 0.0;
        }
        (self.fft)(self.data.as_mut_slice());

        for i in 0..self.data.len()/2 + self.data.len() % 2
        {
            self.data[i].re=0.0;
            self.data[i].im=0.0;
        }
        for i in self.data.len()/2 + self.data.len() % 2 .. self.data.len()
        {
            self.data[i].re=self.data[i].re*2.0;
            self.data[i].im=self.data[i].im*2.0;
        }

        (self.ifft)(self.data.as_mut_slice());
    }
    pub fn inst_freq (&mut self, out: &mut [f32], sample_freq: f32) {
        assert!((self.len==out.len()));
        self.ht();
        out[0] = num_traits::float::Float::abs(num_traits::float::Float::atan(self[1].im/self[1].re)-num_traits::float::Float::atan(self[0].im/self[0].re));
        for i in 1..self.len{
            out[i] = num_traits::float::Float::abs(num_traits::float::Float::atan(self[i].im/self[i].re)-num_traits::float::Float::atan(self[i-1].im/self[i-1].re));
        }
        freq_normalisation(out, sample_freq);
    }
    pub fn inst_freq_ampl (&mut self, out_freq: &mut [f32], out_ampl: &mut [f32], sample_freq: f32){
        self.ht();
        assert!((self.len==out_freq.len()));
        assert!((self.len==out_ampl.len()));
        out_freq[0] = num_traits::float::Float::abs(num_traits::float::Float::atan(self[1].im/self[1].re)-num_traits::float::Float::atan(self[0].im/self[0].re));
        out_ampl[0] = num_traits::float::Float::sqrt(self[0].re.powi(2) + self[0].im.powi(2));
        for i in 1..self.len{
            out_freq[i] = num_traits::float::Float::abs(num_traits::float::Float::atan(self[i].im/self[i].re)-num_traits::float::Float::atan(self[i-1].im/self[i-1].re));
            out_ampl[i] = num_traits::float::Float::sqrt(self[i].re.powi(2) + self[i].im.powi(2));
        }
        freq_normalisation(out_freq, sample_freq);
    }
    pub fn inst_freq_extremums(&mut self, out_freq: &mut [f32], sample_freq: f32){
        self.extremums_vec.clear();
        self.extremums_vec.clear();
        self.extremums_vec.push(0);
        for i in 1..self.len-1{
            if (self[i].re>self[i-1].re && self[i].re>=self[i+1].re) || (self[i].re>=self[i-1].re && self[i].re>self[i+1].re) ||
                (self[i].re<self[i-1].re && self[i].re<=self[i+1].re) || (self[i].re<=self[i-1].re && self[i].re<self[i+1].re){
                if self.extremums_vec.len()>1{
                    if (num_traits::float::Float::abs(self[self.extremums_vec[self.extremums_vec.len()-2]].re-
                        self[self.extremums_vec[self.extremums_vec.len()-1]].re)>self.kvant &&
                        num_traits::float::Float::abs(self[self.extremums_vec[self.extremums_vec.len()-1]].re-
                        self[i].re)>self.kvant) == false{
                        let index = self.extremums_vec.len()-1;
                        self.extremums_vec[index] = i;
                    }
                    else {
                        self.extremums_vec.push(i);
                    }
                }
                else {
                    self.extremums_vec.push(i);
                }
            }
        }
        self.extremums_vec.push(self.len-1);
        let mut counter: usize = 0;
        let mut temp: usize = 1;
        let mut temp_old = 0;
        let mut freq = 0.0;
        while counter<self.len-1{
            if temp<self.extremums_vec.len()-1{
                if self.extremums_vec[temp+1]==counter{
                    temp=temp+1;
                }
                if temp!=temp_old{
                    temp_old = temp;
                    freq = sample_freq / 2.0 / ((self.extremums_vec[temp+1] - self.extremums_vec[temp]) as f32);
                }
            }
            else {
                if temp!=temp_old{
                    temp_old = temp;
                    freq = sample_freq / 2.0 / ((self.extremums_vec[temp] - self.extremums_vec[temp-1]) as f32);
                }
            }
            out_freq[counter] = freq;
            counter = counter+1;
        }
    }
    pub fn inst_freq_extremums_averaging(&mut self, out_freq: &mut [f32], sample_freq: f32){
        self.extremums_vec.clear();
        self.extremums_vec.push(0);
        for i in 1..self.len-1{
            if (self[i].re>self[i-1].re && self[i].re>=self[i+1].re) || (self[i].re>=self[i-1].re && self[i].re>self[i+1].re) ||
                (self[i].re<self[i-1].re && self[i].re<=self[i+1].re) || (self[i].re<=self[i-1].re && self[i].re<self[i+1].re){
                if self.extremums_vec.len()>1{
                    if (num_traits::float::Float::abs(self[self.extremums_vec[self.extremums_vec.len()-2]].re-
                        self[self.extremums_vec[self.extremums_vec.len()-1]].re)>self.kvant &&
                        num_traits::float::Float::abs(self[self.extremums_vec[self.extremums_vec.len()-1]].re-
                        self[i].re)>self.kvant) == false{
                        let index = self.extremums_vec.len()-1;
                        self.extremums_vec[index] = i;
                    }
                    else {
                        self.extremums_vec.push(i);
                    }
                }
                else {
                    self.extremums_vec.push(i);
                }
            }
        }
        self.extremums_vec.push(self.len-1);

        self.freq_x.clear();
        self.freq_y.clear();
        self.freq_x.push(0.0);
        self.freq_y.push(sample_freq / 2.0 / ((self.extremums_vec[1] - self.extremums_vec[0]) as f32));

        for i in 0..self.extremums_vec.len()-1{
            self.freq_x.push((self.extremums_vec[i]+(self.extremums_vec[i+1]-self.extremums_vec[i])/2) as f32);
            self.freq_y.push(sample_freq / 2.0 / ((self.extremums_vec[i+1] - self.extremums_vec[i]) as f32));
        }
        self.freq_x.push((self.extremums_vec[self.extremums_vec.len()-2]+(self.extremums_vec[self.extremums_vec.len()-1]-self.extremums_vec[self.extremums_vec.len()-2])) as f32 /2.0);
        self.freq_y.push(sample_freq / 2.0 / ((self.extremums_vec[self.extremums_vec.len()-1] - self.extremums_vec[self.extremums_vec.len()-2]) as f32));

        let mut counter: usize = 0;
        let mut temp: usize = 0;
        while counter<self.len-1{
            if temp==self.freq_x.len()-1{
                out_freq[counter] = self.freq_y[temp] as f32;
            }
            else if (self.freq_x[temp+1]-self.freq_x[temp])==0.0{
                out_freq[counter] = self.freq_y[temp] as f32;
                if self.freq_x[temp+1]<=counter as f32{
                    temp = temp+1;
                }
            }
            else{
                out_freq[counter] = (self.freq_y[temp] * (self.freq_x[temp+1] - counter as f32)
                    + self.freq_y[temp+1] * (counter as f32 - self.freq_x[temp])) /
                    ((self.freq_x[temp+1]-self.freq_x[temp]) as f32);

                if self.freq_x[temp+1]<=counter as f32{
                    temp = temp+1;
                }
            }
            counter = counter + 1;
        }
    }
    pub fn len(&self)->usize{
        self.len
    }
}