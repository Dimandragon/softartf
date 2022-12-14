extern crate core;

use rustfft::{num_complex::Complex};
use crate::structs::FrSqIFCompute::FrSqIFCompute;
use crate::structs::ImfsFrame::ImfsFrame;
use crate::structs::FreqReprArr::FreqReprArr;
use crate::structs::FreqReprMap::FreqReprMap;
use crate::structs::TfFrameArr::TfFrameArr;
use crate::structs::TfFrameMap::TfFrameMap;
use crate::structs::Averaging::Averaging;
use crate::structs::FIFCompute::FIFCompute;
use softartf_enums::*;

pub mod structs;
mod math;
mod test_data;


#[cfg(test)]
mod test;
mod plotterV2;

pub struct FrSqIF{
    frsqif_computing: FrSqIFCompute,
    pub imf_frame: Option<ImfsFrame>,
    pub freq_repr_arr: Option<FreqReprArr>,
    pub freq_repr_map: Option<FreqReprMap>,
    pub tf_frame_arr: Option<TfFrameArr>,
    pub tf_frame_map: Option<TfFrameMap>,

    pub freq_detail_hz: Option<f32>,
    pub freq_detail_period_sec: Option<f32>,
    pub time_kvant_samples: Option<usize>,
    pub time_kvant_seconds: Option<f32>,

    imf_buffer: Option<Vec<Complex<f32>>>,
    inst_freq: Option<Vec<f32>>,
    inst_ampl: Option<Vec<f32>>,

    averaging: Option<Averaging>,

    data_plots: bool,
}

impl FrSqIF {
    pub fn new(planner_in: rustfft::FftPlanner<f32>) -> FrSqIF{
        FrSqIF{
            frsqif_computing: FrSqIFCompute::new(planner_in),
            imf_frame: None,
            freq_repr_arr: None,
            freq_repr_map: None,
            tf_frame_arr: None,
            tf_frame_map: None,

            freq_detail_hz: None,
            freq_detail_period_sec: None,
            time_kvant_samples: None,
            time_kvant_seconds: None,

            imf_buffer: None,
            inst_freq: None,
            inst_ampl: None,

            averaging: None,

            data_plots: false,
        }
    }

    pub fn set_min_freq(&mut self, new_min_freq: f32){
        self.frsqif_computing.min_freq = new_min_freq;
    }
    pub fn set_sample_freq(&mut self, new_sample_freq: f32) {
        self.frsqif_computing.set_sample_freq(new_sample_freq);
    }
    pub fn set_max_iters(&mut self, new_max_iters: usize) {
        self.frsqif_computing.max_iters = Some(new_max_iters);
    }
    pub fn set_filter_pow(&mut self, new_filter_pow: f32) {
        self.frsqif_computing.set_filter_pow(new_filter_pow);
    }
    pub fn set_filter_x(&mut self, new_filter_x: f32) {
        self.frsqif_computing.set_filter_x(new_filter_x);
    }
    pub fn set_signal(&mut self, signal_in: &[f32]) {
        self.frsqif_computing.set_signal(signal_in);
    }
    pub fn set_avg_len_mn(&mut self, new_avg_len_mn: f32){
        self.frsqif_computing.set_avg_len_mn(new_avg_len_mn);
    }
    pub fn set_frsqif_computing_plots(&mut self, plots: bool){
        self.frsqif_computing.plots = plots;
    }
    pub fn set_data_plots(&mut self, plots: bool){
        self.data_plots = plots;
    }

    pub fn config_averaging(&mut self){
        self.averaging = Some(Averaging::new());
    }
    pub fn set_averaging_run(&mut self, run: bool){
        self.averaging.as_mut().unwrap().set_run(run);
    }
    pub fn set_averaging_window_len(&mut self, new_averaging_window_len: usize){
        self.averaging.as_mut().unwrap().set_averaging_window_len(new_averaging_window_len);
    }
    pub fn set_averaging_freq_desp_averaging_len_mn(&mut self, new_freq_desp_averaging_len_mn: f32){
        self.averaging.as_mut().unwrap().set_freq_desp_averaging_len_mn(new_freq_desp_averaging_len_mn);
    }
    pub fn set_averaging_freq_desp_averaging(&mut self, new_freq_desp_averaging: bool){
        self.averaging.as_mut().unwrap().set_freq_desp_averaging(new_freq_desp_averaging);
    }
    pub fn set_inst_freq_type_for_resampling(&mut self, inst_freq_type_for_resampling: InstFreqTypeForResampling){
        self.frsqif_computing.set_inst_freq_type_for_resampling(inst_freq_type_for_resampling);
    }
    pub fn deconfig_averaging(&mut self){
        self.averaging = None;
    }
    pub fn one_iter(&mut self){
        self.frsqif_computing.one_iter();
    }

    pub fn set_time_kvant_samples(&mut self, kvant_samples: usize){
        self.time_kvant_samples = Some(kvant_samples);
        self.time_kvant_seconds = None;
    }
    pub fn set_time_kvant_seconds(&mut self, kvant_samples: f32){
        self.time_kvant_seconds = Some(kvant_samples);
        self.time_kvant_samples = None;
    }
    pub fn set_freq_detail_hz(&mut self, freq_detail_hz: f32){
        self.freq_detail_hz = Some(freq_detail_hz);
        self.freq_detail_period_sec = None;
    }
    pub fn set_freq_detail_sec(&mut self, freq_detail_sec: f32){
        self.freq_detail_period_sec = Some(freq_detail_sec);
        self.freq_detail_hz = None;
    }

    pub fn config_imf_buffer(&mut self){
        if self.imf_buffer.is_none(){
            self.imf_buffer = Some(std::vec::Vec::with_capacity(self.frsqif_computing.signal_len));
        }
        else{
            if self.imf_buffer.as_ref().unwrap().len()<self.frsqif_computing.signal_len{
                for _i in self.imf_buffer.as_ref().unwrap().len()..self.frsqif_computing.signal_len{
                    self.imf_buffer.as_mut().unwrap().push(Complex{re: 0.0, im: 0.0});
                }
            }
            if self.imf_buffer.as_mut().unwrap().len()>self.frsqif_computing.signal_len{
                self.imf_buffer.as_mut().unwrap().truncate(self.frsqif_computing.signal_len);
            }
        }
    }
    pub fn deconfig_imf_buffer(&mut self){
        self.imf_buffer = None;
    }

    pub fn config_imfs_frame(&mut self){
        self.imf_frame = Some(ImfsFrame::new());
    }
    pub fn add_to_imf_frame(&mut self){
        self.imf_frame.as_mut().unwrap().add_data(&mut self.frsqif_computing);
    }
    pub fn deconfig_imfs_frame(&mut self){
        self.imf_frame = None;
    }

    pub fn config_freq_repr_arr(&mut self){
        self.freq_repr_arr = Some(FreqReprArr::new(&self.frsqif_computing, self.freq_detail_hz, self.freq_detail_period_sec));
    }
    pub fn add_to_freq_repr_arr(&mut self){
        if self.inst_freq.is_none(){
            self.inst_freq = Some(Vec::new());
        }
        if self.inst_ampl.is_none(){
            self.inst_ampl = Some(Vec::new());
        }
        assert!(self.imf_buffer.is_some());
        self.freq_repr_arr.as_mut().unwrap().add_data(&mut self.frsqif_computing, self.freq_detail_hz,
                                                      self.freq_detail_period_sec, self.inst_freq.as_mut().unwrap(),
                                                      self.inst_ampl.as_mut().unwrap(), self.imf_buffer.as_mut().unwrap());

    }
    pub fn deconfig_freq_repr_arr(&mut self){
        self.freq_repr_arr = None;
    }

    pub fn config_freq_repr_map(&mut self){
        self.freq_repr_map = Some(FreqReprMap::new());
    }
    pub fn add_to_freq_repr_map(&mut self){
        if self.inst_freq.is_none(){
            self.inst_freq = Some(Vec::new());
        }
        if self.inst_ampl.is_none(){
            self.inst_ampl = Some(Vec::new());
        }
        self.freq_repr_map.as_mut().unwrap().add_data(&mut self.frsqif_computing, self.freq_detail_hz,
                                                      self.freq_detail_period_sec, self.inst_freq.as_mut().unwrap(),
                                                      self.inst_ampl.as_mut().unwrap(), self.imf_buffer.as_mut().unwrap());
    }
    pub fn deconfig_freq_repr_map(&mut self){
        self.freq_repr_map = None;
    }

    pub fn config_tf_frame_arr(&mut self){
        self.tf_frame_arr = Some(TfFrameArr::new(&self.frsqif_computing,
                                                 self.freq_detail_hz, self.freq_detail_period_sec, self.time_kvant_samples, self.time_kvant_seconds));
    }
    pub fn add_to_tf_frame_arr(&mut self){
        if self.inst_freq.is_none(){
            self.inst_freq = Some(Vec::new());
        }
        if self.inst_ampl.is_none(){
            self.inst_ampl = Some(Vec::new());
        }
        self.tf_frame_arr.as_mut().unwrap().add_data(&mut self.frsqif_computing,
                                                     self.freq_detail_hz, self.freq_detail_period_sec, self.time_kvant_samples, self.time_kvant_seconds,
                                                     self.inst_freq.as_mut().unwrap(), self.inst_ampl.as_mut().unwrap(), self.imf_buffer.as_mut().unwrap());
    }
    pub fn deconfig_tf_frame_arr(&mut self){
        self.tf_frame_arr = None;
    }

    pub fn config_tf_frame_map(&mut self){
        self.tf_frame_map = Some(TfFrameMap::new(&self.frsqif_computing, self.time_kvant_samples, self.time_kvant_seconds));

    }
    pub fn add_to_tf_frame_map(&mut self){
        if self.inst_freq.is_none(){
            self.inst_freq = Some(Vec::new());
        }
        if self.inst_ampl.is_none(){
            self.inst_ampl = Some(Vec::new());
        }
        self.tf_frame_map.as_mut().unwrap().add_data(&mut self.frsqif_computing,
                                                     self.freq_detail_hz, self.freq_detail_period_sec, self.time_kvant_samples, self.time_kvant_seconds,
                                                     self.inst_freq.as_mut().unwrap(), self.inst_ampl.as_mut().unwrap(), self.imf_buffer.as_mut().unwrap());
    }
    pub fn deconfig_tf_frame_map(&mut self){
        if self.inst_freq.is_none(){
            self.inst_freq = Some(Vec::new());
        }
        self.tf_frame_map = None;
    }

    /*pub fn average_inst_freq_and_ampl(&mut self){
        assert!(self.inst_freq.is_some());
        assert!(self.inst_ampl.is_some());
        let temp_len = self.inst_freq.as_ref().unwrap().len();
        if self.inst_freq.as_ref().unwrap().len()>temp_len{
            self.inst_freq.as_mut().unwrap().truncate(temp_len);
        }
        if self.inst_ampl.as_ref().unwrap().len()>temp_len{
            self.inst_ampl.as_mut().unwrap().truncate(temp_len);
        }
        self.averaging.as_mut().unwrap().average_inst_freq(&mut self.inst_freq[0..self.contains_len], &mut self.new_freq_conv[0..self.contains_len]);
        self.averaging.as_mut().unwrap().average(self.inst_ampl.as_mut().unwrap().as_mut_slice(),
                                &self.frsqif_computing, self.inst_freq.as_mut().unwrap());
    }*/

    pub fn reconfig(&mut self){
        if self.imf_frame.is_some(){
            self.imf_frame.as_mut().unwrap().clear();
        }
        if self.freq_repr_arr.is_some(){
            self.freq_repr_arr.as_mut().unwrap().reconfig(&self.frsqif_computing,
                                                          self.freq_detail_hz, self.freq_detail_period_sec);
        }
        if self.freq_repr_map.is_some(){
            self.freq_repr_map.as_mut().unwrap().zeroes();
        }
        if self.tf_frame_arr.is_some(){
            self.tf_frame_arr.as_mut().unwrap().reconfig(&self.frsqif_computing,
                                                         self.freq_detail_hz, self.freq_detail_period_sec,
                                                         self.time_kvant_samples, self.time_kvant_seconds);
        }
        if self.tf_frame_map.is_some(){
            self.tf_frame_map.as_mut().unwrap().reconfig(&self.frsqif_computing,
                                                         self.time_kvant_samples, self.time_kvant_seconds);
        }
    }

    pub fn compute_single_data_repr(&mut self){
        if self.imf_frame.is_some(){
            self.add_to_imf_frame();
        }
        if self.freq_repr_arr.is_some(){
            self.add_to_freq_repr_arr();
        }
        if self.freq_repr_map.is_some(){
            self.add_to_freq_repr_map();
        }
        if self.tf_frame_arr.is_some(){
            self.add_to_tf_frame_arr();
        }
        if self.tf_frame_map.is_some(){
            self.add_to_tf_frame_map();
        }
    }

    pub fn set_imfs_frame_detail_plotting(&mut self, imfs_frame_detail_plotting: bool){
        if self.imf_frame.is_some(){
            self.imf_frame.as_mut().unwrap().set_detail_plotting(imfs_frame_detail_plotting);
        }
    }

    pub fn plot(&mut self){
        if self.data_plots == true{
            if self.imf_frame.is_some(){
                self.imf_frame.as_ref().unwrap().plot();
            }
            if self.freq_repr_arr.is_some(){
                self.freq_repr_arr.as_ref().unwrap().plot();
            }
            if self.freq_repr_map.is_some(){
                self.freq_repr_map.as_ref().unwrap().plot();
            }
            if self.tf_frame_map.is_some(){
                self.tf_frame_map.as_mut().unwrap().plot();
            }
        }
    }

    pub fn set_tf_plot_pow_base(&mut self, plot_pow_base: Option<f64>){
        if self.tf_frame_map.is_some(){
            self.tf_frame_map.as_mut().unwrap().set_plotting_pow_base(plot_pow_base);
        }
    }
    pub fn set_tf_plot_log_base(&mut self, plot_log_base: Option<f64>){
        if self.tf_frame_map.is_some(){
            self.tf_frame_map.as_mut().unwrap().set_plotting_log_base(plot_log_base);
        }
    }

    pub fn compute_frsqif(&mut self){
        while self.frsqif_computing.continue_status == true{
            self.one_iter();
            self.compute_single_data_repr();
        }
        self.plot();
    }
    pub fn set_round(&mut self, round: Option<f32>){
        self.frsqif_computing.set_round(round);
    }

    pub fn set_inst_freq_type(&mut self, inst_freq_type: InstFreqTypeForResampling){
        if self.tf_frame_map.is_some(){
            self.tf_frame_map.as_mut().unwrap().set_inst_freq_type(inst_freq_type);
        }
    }
    pub fn set_inst_ampl_type(&mut self, inst_ampl_type: InstAmplType){
        if self.tf_frame_map.is_some(){
            self.tf_frame_map.as_mut().unwrap().set_inst_ampl_type(inst_ampl_type);
        }
    }


    pub fn default_computing (&mut self, signal: &[f32], sample_freq: f32){
        self.set_signal(signal);
        self.set_filter_pow(1.0);
        self.set_filter_x(0.9);
        self.set_inst_freq_type_for_resampling(InstFreqTypeForResampling::ExtremumsAveraging);
        self.set_round(Some(0.01));
        //self.set_round(Some(0.2));
        self.set_freq_detail_hz(2.5);
        self.set_time_kvant_samples(10);

        self.set_sample_freq(sample_freq);
        self.config_imfs_frame();
        self.config_tf_frame_map();
        self.set_tf_plot_log_base(Some(2.0));
        self.set_tf_plot_pow_base(Some(1.0));
        self.set_inst_freq_type(InstFreqTypeForResampling::SimpleAveraging);
        self.set_inst_ampl_type(InstAmplType::SimpleAveraging);

        self.set_imfs_frame_detail_plotting(true);
        self.set_min_freq(5.0);
        self.config_imf_buffer();
        self.set_avg_len_mn(0.01);
        self.set_max_iters(15);
        self.set_data_plots(true);
        self.set_frsqif_computing_plots(true);
        self.compute_frsqif();
    }
}



















pub struct FIF{
    fif_computing: FIFCompute,
    pub imf_frame: Option<ImfsFrame>,
    pub freq_repr_arr: Option<FreqReprArr>,
    pub freq_repr_map: Option<FreqReprMap>,
    pub tf_frame_arr: Option<TfFrameArr>,
    pub tf_frame_map: Option<TfFrameMap>,

    pub freq_detail_hz: Option<f32>,
    pub freq_detail_period_sec: Option<f32>,
    pub time_kvant_samples: Option<usize>,
    pub time_kvant_seconds: Option<f32>,

    imf_buffer: Option<Vec<Complex<f32>>>,
    inst_freq: Option<Vec<f32>>,
    inst_ampl: Option<Vec<f32>>,

    averaging: Option<Averaging>,

    data_plots: bool,
}

impl FIF {
    pub fn new(planner_in: rustfft::FftPlanner<f32>) -> FIF{
        FIF{
            fif_computing: FIFCompute::new(planner_in),
            imf_frame: None,
            freq_repr_arr: None,
            freq_repr_map: None,
            tf_frame_arr: None,
            tf_frame_map: None,

            freq_detail_hz: None,
            freq_detail_period_sec: None,
            time_kvant_samples: None,
            time_kvant_seconds: None,

            imf_buffer: None,
            inst_freq: None,
            inst_ampl: None,

            averaging: None,

            data_plots: false,
        }
    }

    pub fn set_min_freq(&mut self, new_min_freq: f32){
        self.fif_computing.min_freq = new_min_freq;
    }
    pub fn set_sample_freq(&mut self, new_sample_freq: f32) {
        self.fif_computing.set_sample_freq(new_sample_freq);
    }
    pub fn set_max_iters(&mut self, new_max_iters: usize) {
        self.fif_computing.max_iters = Some(new_max_iters);
    }
    pub fn set_filter_pow(&mut self, new_filter_pow: f32) {
        self.fif_computing.set_filter_pow(new_filter_pow);
    }
    pub fn set_filter_x(&mut self, new_filter_x: f32) {
        self.fif_computing.set_filter_x(new_filter_x);
    }
    pub fn set_signal(&mut self, signal_in: &[f32]) {
        self.fif_computing.set_signal(signal_in);
    }
    pub fn set_avg_len_mn(&mut self, new_avg_len_mn: f32){
        self.fif_computing.set_avg_len_mn(new_avg_len_mn);
    }
    pub fn set_fif_computing_plots(&mut self, plots: bool){
        self.fif_computing.plots = plots;
    }
    pub fn set_data_plots(&mut self, plots: bool){
        self.data_plots = plots;
    }
    pub fn set_fif_computing_len_filter_draft_option(&mut self, len_filter_draft_option: LenFilterDraftOption){
        self.fif_computing.len_filter_draft_option = len_filter_draft_option;
    }

    pub fn config_averaging(&mut self){
        self.averaging = Some(Averaging::new());
    }
    pub fn set_averaging_run(&mut self, run: bool){
        self.averaging.as_mut().unwrap().set_run(run);
    }
    pub fn set_averaging_window_len(&mut self, new_averaging_window_len: usize){
        self.averaging.as_mut().unwrap().set_averaging_window_len(new_averaging_window_len);
    }
    pub fn set_averaging_freq_desp_averaging_len_mn(&mut self, new_freq_desp_averaging_len_mn: f32){
        self.averaging.as_mut().unwrap().set_freq_desp_averaging_len_mn(new_freq_desp_averaging_len_mn);
    }
    pub fn set_averaging_freq_desp_averaging(&mut self, new_freq_desp_averaging: bool){
        self.averaging.as_mut().unwrap().set_freq_desp_averaging(new_freq_desp_averaging);
    }
    pub fn set_inst_freq_type_for_resampling(&mut self, inst_freq_type_for_resampling: InstFreqTypeForResampling){
        self.fif_computing.set_inst_freq_type_for_resampling(inst_freq_type_for_resampling);
    }
    pub fn deconfig_averaging(&mut self){
        self.averaging = None;
    }
    pub fn one_iter(&mut self){
        self.fif_computing.one_iter();
    }

    pub fn set_time_kvant_samples(&mut self, kvant_samples: usize){
        self.time_kvant_samples = Some(kvant_samples);
        self.time_kvant_seconds = None;
    }
    pub fn set_time_kvant_seconds(&mut self, kvant_samples: f32){
        self.time_kvant_seconds = Some(kvant_samples);
        self.time_kvant_samples = None;
    }
    pub fn set_freq_detail_hz(&mut self, freq_detail_hz: f32){
        self.freq_detail_hz = Some(freq_detail_hz);
        self.freq_detail_period_sec = None;
    }
    pub fn set_freq_detail_sec(&mut self, freq_detail_sec: f32){
        self.freq_detail_period_sec = Some(freq_detail_sec);
        self.freq_detail_hz = None;
    }

    pub fn config_imf_buffer(&mut self){
        if self.imf_buffer.is_none(){
            self.imf_buffer = Some(std::vec::Vec::with_capacity(self.fif_computing.signal_len));
        }
        else{
            if self.imf_buffer.as_ref().unwrap().len()<self.fif_computing.signal_len{
                for _i in self.imf_buffer.as_ref().unwrap().len()..self.fif_computing.signal_len{
                    self.imf_buffer.as_mut().unwrap().push(Complex{re: 0.0, im: 0.0});
                }
            }
            if self.imf_buffer.as_mut().unwrap().len()>self.fif_computing.signal_len{
                self.imf_buffer.as_mut().unwrap().truncate(self.fif_computing.signal_len);
            }
        }
    }
    pub fn deconfig_imf_buffer(&mut self){
        self.imf_buffer = None;
    }

    pub fn config_imfs_frame(&mut self){
        self.imf_frame = Some(ImfsFrame::new());
    }
    pub fn add_to_imf_frame(&mut self){
        self.imf_frame.as_mut().unwrap().add_data_fif(&mut self.fif_computing);
    }
    pub fn deconfig_imfs_frame(&mut self){
        self.imf_frame = None;
    }

    pub fn config_freq_repr_arr(&mut self){
        self.freq_repr_arr = Some(FreqReprArr::new_fif(&self.fif_computing, self.freq_detail_hz, self.freq_detail_period_sec));
    }
    pub fn add_to_freq_repr_arr(&mut self){
        if self.inst_freq.is_none(){
            self.inst_freq = Some(Vec::new());
        }
        if self.inst_ampl.is_none(){
            self.inst_ampl = Some(Vec::new());
        }
        assert!(self.imf_buffer.is_some());
        self.freq_repr_arr.as_mut().unwrap().add_data_fif(&mut self.fif_computing, self.freq_detail_hz,
                                                      self.freq_detail_period_sec, self.inst_freq.as_mut().unwrap(),
                                                      self.inst_ampl.as_mut().unwrap(), self.imf_buffer.as_mut().unwrap());

    }
    pub fn deconfig_freq_repr_arr(&mut self){
        self.freq_repr_arr = None;
    }

    pub fn config_freq_repr_map(&mut self){
        self.freq_repr_map = Some(FreqReprMap::new());
    }
    pub fn add_to_freq_repr_map(&mut self){
        if self.imf_frame.is_some(){
            self.add_to_imf_frame();
        }
        if self.freq_repr_arr.is_some(){
            self.add_to_freq_repr_arr();
        }
        self.freq_repr_map.as_mut().unwrap().add_data_fif(&mut self.fif_computing, self.freq_detail_hz,
                                                      self.freq_detail_period_sec, self.inst_freq.as_mut().unwrap(),
                                                      self.inst_ampl.as_mut().unwrap(), self.imf_buffer.as_mut().unwrap());
    }
    pub fn deconfig_freq_repr_map(&mut self){
        self.freq_repr_map = None;
    }

    pub fn config_tf_frame_arr(&mut self){
        self.tf_frame_arr = Some(TfFrameArr::new_fif(&self.fif_computing,
                                                 self.freq_detail_hz, self.freq_detail_period_sec, self.time_kvant_samples, self.time_kvant_seconds));
    }
    pub fn add_to_tf_frame_arr(&mut self){
        if self.imf_frame.is_some(){
            self.add_to_imf_frame();
        }
        if self.freq_repr_arr.is_some(){
            self.add_to_freq_repr_arr();
        }
        self.tf_frame_arr.as_mut().unwrap().add_data_fif(&mut self.fif_computing,
                                                     self.freq_detail_hz, self.freq_detail_period_sec, self.time_kvant_samples, self.time_kvant_seconds,
                                                     self.inst_freq.as_mut().unwrap(), self.inst_ampl.as_mut().unwrap(), self.imf_buffer.as_mut().unwrap());
    }
    pub fn deconfig_tf_frame_arr(&mut self){
        self.tf_frame_arr = None;
    }

    pub fn config_tf_frame_map(&mut self){
        self.tf_frame_map = Some(TfFrameMap::new_fif(&self.fif_computing, self.time_kvant_samples, self.time_kvant_seconds));
    }
    pub fn add_to_tf_frame_map(&mut self){
        if self.imf_frame.is_some(){
            self.add_to_imf_frame();
        }
        if self.freq_repr_arr.is_some(){
            self.add_to_freq_repr_arr();
        }
        self.tf_frame_map.as_mut().unwrap().add_data_fif(&mut self.fif_computing,
                                                     self.freq_detail_hz, self.freq_detail_period_sec, self.time_kvant_samples, self.time_kvant_seconds,
                                                     self.inst_freq.as_mut().unwrap(), self.inst_ampl.as_mut().unwrap(), self.imf_buffer.as_mut().unwrap());
    }
    pub fn deconfig_tf_frame_map(&mut self){
        self.tf_frame_map = None;
    }

    /*pub fn average_inst_freq_and_ampl(&mut self){
        assert!(self.inst_freq.is_some());
        assert!(self.inst_ampl.is_some());
        let temp_len = self.inst_freq.as_ref().unwrap().len();
        if self.inst_freq.as_ref().unwrap().len()>temp_len{
            self.inst_freq.as_mut().unwrap().truncate(temp_len);
        }
        if self.inst_ampl.as_ref().unwrap().len()>temp_len{
            self.inst_ampl.as_mut().unwrap().truncate(temp_len);
        }
        self.averaging.as_mut().unwrap().average_inst_freq(&mut self.inst_freq[0..self.contains_len], &mut self.new_freq_conv[0..self.contains_len]);
        self.averaging.as_mut().unwrap().average(self.inst_ampl.as_mut().unwrap().as_mut_slice(),
                                &self.frsqif_computing, self.inst_freq.as_mut().unwrap());
    }*/

    pub fn reconfig(&mut self){
        if self.imf_frame.is_some(){
            self.imf_frame.as_mut().unwrap().clear();
        }
        if self.freq_repr_arr.is_some(){
            self.freq_repr_arr.as_mut().unwrap().reconfig_fif(&self.fif_computing,
                                                          self.freq_detail_hz, self.freq_detail_period_sec);
        }
        if self.freq_repr_map.is_some(){
            self.freq_repr_map.as_mut().unwrap().zeroes();
        }
        if self.tf_frame_arr.is_some(){
            self.tf_frame_arr.as_mut().unwrap().reconfig_fif(&self.fif_computing,
                                                         self.freq_detail_hz, self.freq_detail_period_sec,
                                                         self.time_kvant_samples, self.time_kvant_seconds);
        }
        if self.tf_frame_map.is_some(){
            self.tf_frame_map.as_mut().unwrap().reconfig_fif(&self.fif_computing,
                                                         self.time_kvant_samples, self.time_kvant_seconds);
        }
    }

    pub fn compute_single_data_repr(&mut self){
        if self.imf_frame.is_some(){
            self.add_to_imf_frame();
        }
        if self.freq_repr_arr.is_some(){
            self.add_to_freq_repr_arr();
        }
        if self.freq_repr_map.is_some(){
            self.add_to_freq_repr_map();
        }
        if self.tf_frame_arr.is_some(){
            self.add_to_tf_frame_arr();
        }
        if self.tf_frame_map.is_some(){
            self.add_to_tf_frame_map();
        }
    }

    pub fn set_imfs_frame_detail_plotting(&mut self, imfs_frame_detail_plotting: bool){
        if self.imf_frame.is_some(){
            self.imf_frame.as_mut().unwrap().set_detail_plotting(imfs_frame_detail_plotting);
        }
    }

    pub fn plot(&mut self){
        if self.data_plots == true{
            if self.imf_frame.is_some(){
                self.imf_frame.as_ref().unwrap().plot();
            }
            if self.freq_repr_arr.is_some(){
                self.freq_repr_arr.as_ref().unwrap().plot();
            }
            if self.freq_repr_map.is_some(){
                self.freq_repr_map.as_ref().unwrap().plot();
            }
            if self.tf_frame_map.is_some(){
                self.tf_frame_map.as_mut().unwrap().plot();
            }
        }
    }

    pub fn set_tf_plot_pow_base(&mut self, plot_pow_base: Option<f64>){
        if self.tf_frame_map.is_some(){
            self.tf_frame_map.as_mut().unwrap().set_plotting_pow_base(plot_pow_base);
        }
    }
    pub fn set_tf_plot_log_base(&mut self, plot_log_base: Option<f64>){
        if self.tf_frame_map.is_some(){
            self.tf_frame_map.as_mut().unwrap().set_plotting_log_base(plot_log_base);
        }
    }

    pub fn compute_frsqif(&mut self){
        while self.fif_computing.continue_status == true{
            self.one_iter();
            self.compute_single_data_repr();
        }
        self.plot();
    }
    pub fn set_round(&mut self, round: Option<f32>){
        self.fif_computing.set_round(round);
    }

    pub fn default_computing (&mut self, signal: &[f32], sample_freq: f32){
        self.set_signal(signal);
        self.set_filter_pow(1.0);
        self.set_filter_x(1.6);
        self.set_inst_freq_type_for_resampling(InstFreqTypeForResampling::ExtremumsAveraging);
        self.set_round(Some(0.01));
        //self.set_round(Some(0.2));
        self.set_freq_detail_hz(0.5);
        self.set_time_kvant_samples(5);
        self.set_fif_computing_len_filter_draft_option(LenFilterDraftOption::Minimum);

        self.set_sample_freq(sample_freq);
        self.config_imfs_frame();
        self.config_freq_repr_arr();
        self.config_freq_repr_map();
        //self.config_tf_frame_arr();
        self.config_tf_frame_map();
        self.set_tf_plot_log_base(Some(2.0));
        self.set_tf_plot_pow_base(Some(1.0));
        self.set_imfs_frame_detail_plotting(false);
        self.set_min_freq(5.0);
        self.config_imf_buffer();
        self.set_avg_len_mn(0.01);
        self.set_max_iters(100);
        self.set_data_plots(true);
        self.set_fif_computing_plots(true);
        self.compute_frsqif();
    }
}


