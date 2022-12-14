use num_complex::Complex;
use std::ops::{Index, IndexMut};
use crate::structs::FrSqIFCompute::FrSqIFCompute;
use crate::math::resampling::back_resampling;


pub struct ImfsFrame{
    pub data: Vec<Vec<Complex<f32>>>,
    pub imfs_k: usize,
}

impl ImfsFrame{
    pub fn new()->ImfsFrame{
        ImfsFrame{
            data: Vec::new(),
            imfs_k: 0,
        }
    }
    pub fn add_data(&mut self, frsqif: &mut FrSqIFCompute){
        if frsqif.continue_status == true{
            self.data.push(Vec::with_capacity(frsqif.signal_len));
            back_resampling(&frsqif.signal_image[0..frsqif.contains_len],
                            &mut self.data[self.imfs_k],
                            &frsqif.new_freq_conv[0..frsqif.contains_len],
                            frsqif.signal_len);
            self.imfs_k = self.imfs_k + 1;
        }
        else {
            self.data.push(Vec::with_capacity(frsqif.signal_len));
            back_resampling(&frsqif.signal.get_slice(),
                            &mut self.data[self.imfs_k],
                            &frsqif.new_freq_conv[0..frsqif.contains_len],
                            frsqif.signal_len);
            self.imfs_k = self.imfs_k + 1;
        }
    }
    pub fn len(&self)->usize{
        self.data.len()
    }
    pub fn clear(&mut self){
        for i in 0..self.imfs_k{
            self.data[i].clear();
        }
    }
    /*pub fn add_data_fif(&mut self, fif_computing: &mut crate::structs::FIFCompute::FIFCompute){
        self.data.push(Vec::with_capacity(fif_computing.signal_len));
        if fif_computing.continue_status{
            self.data[self.imfs_k].clear();
            for i in 0..fif_computing.signal.len(){
                self.data[self.imfs_k].push(fif_computing.signal_image[i]);
            }
        }
        else{
            self.data[self.imfs_k].clear();
            for i in 0..fif_computing.signal.len(){
                self.data[self.imfs_k].push(fif_computing.signal[i]);
            }
        }
        self.imfs_k = self.imfs_k + 1;
    }*/
}

impl Index<usize> for ImfsFrame{
    type Output = Vec<Complex<f32>>;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.imfs_k);
        &self.data[index]
    }
}
impl IndexMut<usize> for ImfsFrame{
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert! (index<self.len());
        &mut self.data[index]
    }
}