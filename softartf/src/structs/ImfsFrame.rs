use rustfft::{num_complex::Complex};
use std::ops::{Index, IndexMut};
use crate::structs::FrSqIFCompute::FrSqIFCompute;
use crate::math::resampling::back_resampling;


pub struct ImfsFrame{
    pub data: Vec<Vec<Complex<f32>>>,
    pub imfs_k: usize,
    pub detail_plotting: bool,
}

impl ImfsFrame{
    pub fn new()->ImfsFrame{
        ImfsFrame{
            data: Vec::new(),
            imfs_k: 0,
            detail_plotting: false,
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
    pub fn set_detail_plotting(&mut self, detail_plotting: bool){
        self.detail_plotting = detail_plotting;
    }
    pub fn plot(&self){
        /*for i in 0..self.len(){
            let mut name: String = String::new();
            name.push_str("imf");
            name.push_str(& format!("{}", i));
            name.push_str(".svg");
            println!("{}", self.data[i].len());
        }*/
        if self.detail_plotting == false{
            let mut plotting_vec: Vec<&Vec<Complex<f32>>> = Vec::with_capacity(self.imfs_k);
            for i in 0..self.len(){
                plotting_vec.push(&self.data[i]);
            }
            crate::plotterV2::plot_lines_complex_ignore(&plotting_vec
                                                        ,"images/imfs_data.png"
                                                        ,(3000,2500)
                                                        ,Some("IMFs")
                                                        ,None);
        }
        else {
            for i in 0..self.len(){
                let mut name: String = String::new();
                name.push_str("imf");
                name.push_str(& format!("{}", i));
                name.push_str(".png");
                let mut path = String::new();
                path.push_str("images/");
                path.push_str("imf");
                path.push_str(& format!("{}", i));
                path.push_str(".png");

                crate::plotterV2::plot_lines_complex_ignore(&vec![&self.data[i]]
                                                            ,&path
                                                            ,(1000,800)
                                                            ,Some(&name)
                                                            ,None);
            }
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
    pub fn add_data_fif(&mut self, fif_computing: &mut crate::structs::FIFCompute::FIFCompute){
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
    }
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