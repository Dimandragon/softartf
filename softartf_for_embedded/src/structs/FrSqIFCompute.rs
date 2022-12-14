use num_complex::Complex;
use crate::structs::PaddedSignalComplex::PaddedSignalComplex;
use crate::math::resampling::simple_resampling;
use crate::math::filters::generate_simple_mask;
use crate::structs::Averaging::Averaging;
use softartf_enums::InstFreqTypeForResampling;


pub struct FrSqIFCompute {
    fft: fn(&mut[Complex<f32>]),
    ifft: fn(&mut[Complex<f32>]),
    //fft implementation
    pub signal: PaddedSignalComplex,
    //container which uses for containing the signal data or the temporary signal representation
    pub signal_image: std::vec::Vec<Complex<f32>>,
    //container which uses for containing the resampled signal data or the resampled temporary signal representation
    pub signal_len: usize,
    //variable which uses for containing the len of signal
    pub signal_image_len: usize,
    //variable which uses for containing the len of resampled signal
    pub contains_len: usize,
    //variable which uses for containing the instant contains lenght
    pub sample_freq: f32,
    //variable which uses for containing the sampling frequency of signal
    pub inst_freq: Vec<f32>,
    //container which uses for containing the instantaneous frequency of signal or temporary signal representation
    pub filter: std::vec::Vec<Complex<f32>>,
    //container which uses for containing the data of temporary filter
    pub filter_len: usize,
    //variable which uses for containing the data of temporary filter len
    pub new_freq_conv: std::vec::Vec<f32>,
    //resampled function
    pub new_freq_conv_image: std::vec::Vec<f32>,
    //resampled function buffer
    pub filter_pow: f32,
    //power coefficient for filter computing
    pub filter_x: f32,
    //lenght coefficient for filter computing
    pub continue_status: bool,
    //variable (status) which control the continuation of computing
    pub iters_counter: usize,
    //counter of iterations
    pub min_freq: f32,
    //the point of out the decomposition associated with summary instantaneous frequency of signal
    pub max_iters: Option<usize>,
    //the point of out the decomposition associated with iterations amount
    pub averaging: Averaging,
    //buffer for signal averaging
    clear: bool,
    //marker of new object for first initialisation
    pub inst_freq_type_for_resampling: InstFreqTypeForResampling,
    pub round: Option<f32>,
}

impl FrSqIFCompute{
    pub fn new(fft_in: fn(&mut[Complex<f32>]), ifft_in: fn(&mut[Complex<f32>]))->FrSqIFCompute{
        FrSqIFCompute {
            fft: fft_in,
            ifft: ifft_in,
            signal: PaddedSignalComplex::new(fft_in, ifft_in),
            signal_image: std::vec::Vec::new(),
            signal_len: 0,
            contains_len: 0,

            signal_image_len: 0,
            sample_freq: num_traits::identities::Zero::zero(),
            inst_freq: std::vec::Vec::new(),
            filter: std::vec::Vec::new(),
            filter_len: 0,
            new_freq_conv: std::vec::Vec::new(),
            new_freq_conv_image: std::vec::Vec::new(),
            filter_pow: num_traits::identities::Zero::zero(),
            filter_x: num_traits::identities::Zero::zero(),

            continue_status: true,
            iters_counter: 0,
            min_freq: num_traits::identities::one(),
            max_iters: None,

            averaging: Averaging::new(),

            clear: true,
            inst_freq_type_for_resampling: InstFreqTypeForResampling::Extremums,
            round:  None,
        }
    }
    pub fn set_inst_freq_type_for_resampling(&mut self, inst_freq_type_for_resampling_in: InstFreqTypeForResampling){
        self.inst_freq_type_for_resampling = inst_freq_type_for_resampling_in;
    }
    pub fn set_signal (&mut self, signal_in: &[f32]){
        self.iters_counter = 0;
        self.continue_status = true;
        if self.clear == true{
            self.signal.resize(signal_in.len());
            self.signal_image = std::vec::Vec::with_capacity(signal_in.len()+signal_in.len()/2);//vec![Complex{re: 0.0, im: 0.0}; signal_in.len()+signal_in.len()/2];
            self.signal_len = signal_in.len();
            self.contains_len = signal_in.len();
            self.inst_freq = vec![0.0; signal_in.len()];
            self.filter = vec![Complex{re: 0.0, im: 0.0}; signal_in.len()];
            self.new_freq_conv = vec![1.0; signal_in.len()];
            self.new_freq_conv_image = vec![0.0; signal_in.len()];
            self.clear = false;

            for i in 0..signal_in.len(){
                self.signal[i].re = signal_in[i];
            }
        }
        else{
            self.signal.resize(signal_in.len()+signal_in.len()/2);
            self.signal_image.clear();
            self.signal_image.resize(signal_in.len()+signal_in.len()/2, Complex{re: 0.0, im: 0.0});
            self.signal_len = signal_in.len();
            self.contains_len = signal_in.len();
            self.inst_freq.clear();
            self.inst_freq.resize(signal_in.len(), 0.0);
            self.filter.clear();
            self.filter.resize(signal_in.len(), Complex{re: 0.0, im: 0.0});
            self.new_freq_conv.clear();
            self.new_freq_conv.resize(signal_in.len(), 1.0);
            self.new_freq_conv_image.clear();
            self.new_freq_conv_image.resize(signal_in.len(), 0.0);

            for i in 0..signal_in.len(){
                self.signal[i].re = signal_in[i];
            }
        }
    }
    pub fn set_round(&mut self, round: Option<f32>){
        self.round = round;
    }
    pub fn set_min_freq (&mut self, new_min_freq: f32){
        self.min_freq = new_min_freq;
    }
    pub fn set_avg_len_mn(&mut self, new_avg_len_mn: f32){
        self.averaging.set_freq_desp_averaging_len_mn(new_avg_len_mn);
    }
    pub fn set_max_iters (&mut self, new_max_iters: std::option::Option<usize>){
        self.max_iters = new_max_iters;
    }
    pub fn set_kvant(&mut self, kvant: f32){
        self.signal.set_kvant(kvant);
    }
    pub fn set_sample_freq (&mut self, new_sample_freq: f32){
        self.sample_freq = new_sample_freq;
    }
    pub fn set_filter_pow (&mut self, new_filter_pow: f32){
        self.filter_pow = new_filter_pow;
    }
    pub fn set_filter_x (&mut self, new_filter_x: f32){
        assert!(new_filter_x > 0.0);
        self.filter_x = new_filter_x;
    }
    pub fn resize_after_resampled(&mut self){
        self.contains_resize(self.signal_image_len);
    }
    fn contains_resize(&mut self, new_len: usize){
        self.inst_freq.resize(new_len, 0.0);
        self.filter.resize(new_len, Complex{re: 0.0, im: 0.0});
        self.contains_len = new_len;
    }
    pub fn one_iter(&mut self){
        let zero: f32 = num_traits::identities::Zero::zero();
        assert!(self.sample_freq > zero);
        assert!(self.signal_len > 0);
        assert!(self.filter_x > zero);
        if self.round.is_some(){
            for i in 0..self.signal.len{
                self.signal[i].re = self.signal[i].re.div_euclid(self.round.unwrap())*self.round.unwrap();
            }
        }
        match self.inst_freq_type_for_resampling {
            InstFreqTypeForResampling::Simple => {
                self.signal.inst_freq(&mut self.inst_freq[0..self.contains_len], self.sample_freq * (self.contains_len as f32) / (self.signal_len as f32));
            },
            InstFreqTypeForResampling::Extremums => {
                self.signal.inst_freq_extremums(&mut self.inst_freq[0..self.contains_len], self.sample_freq * (self.contains_len as f32) / (self.signal_len as f32));
            },
            InstFreqTypeForResampling::ExtremumsAveraging => {
                self.signal.inst_freq_extremums_averaging(&mut self.inst_freq[0..self.contains_len], self.sample_freq * (self.contains_len as f32) / (self.signal_len as f32));
            },
            InstFreqTypeForResampling::SimpleAveraging => {
                self.signal.inst_freq(&mut self.inst_freq[0..self.contains_len], self.sample_freq * (self.contains_len as f32) / (self.signal_len as f32));
                self.averaging.average_inst_freq(&mut self.inst_freq[0..self.contains_len], &mut self.new_freq_conv[0..self.contains_len]);
            },
        }
        /*println!(" ");
        println!(" ");
        for i in 0..self.inst_freq.len(){
            print!("{} ", self.inst_freq[i]);
        }*/

        let mut sum: f32 = 0.0;
        let mut min_freq_temp: f32 = std::f32::MAX;
        let mut max_freq_temp: f32 = 0.0;
        for i in 0..self.contains_len{
            sum = sum + self.inst_freq[i]/(self.contains_len as f32);
            if min_freq_temp>self.inst_freq[i]{
                min_freq_temp = self.inst_freq[i];
            }
            if max_freq_temp<self.inst_freq[i]{
                max_freq_temp = self.inst_freq[i];
            }
        }
        //println!("freq: {}", sum);
        if self.max_iters.is_some(){
            if self.max_iters.unwrap()>=self.iters_counter{
                if sum>self.min_freq{
                    self.iters_counter = self.iters_counter+1;

                    for i in 0..self.signal.len{
                        self.signal[i].im = 0.0;
                    }
                    let new_freq:f32 = simple_resampling(self.signal.get_slice(),
                                                         &self.inst_freq[0..self.contains_len],
                                                         &mut self.signal_image,
                                                         &mut self.new_freq_conv,
                                                         &mut self.new_freq_conv_image,
                                                         self.signal_len);

                    self.signal_image_len = self.signal_image.len();
                    self.resize_after_resampled();
                    if new_freq < self.min_freq{
                        self.continue_status = false;
                    }
                    self.signal.set_signal(&self.signal_image);
                    self.filter_len = (self.sample_freq * (self.contains_len as f32) / (self.signal_len as f32) /new_freq*self.filter_x) as usize;
                    if self.filter_len>self.contains_len{
                        self.filter_len = self.contains_len;
                    }
                    self.filter.resize(self.signal.len*2, Complex{re: 0.0, im: 0.0});
                    //print!("{}", self.filter_len);
                    generate_simple_mask(self.filter_len, self.filter_pow, &mut self.filter[0..self.filter_len]);

                    /*println!(" ");
                    for i in 0..self.signal.len{
                        print!("signal_len_before_conv[{}] {} ", i, self.signal[i]);
                    }*/
                    self.signal.conv(&mut self.filter, self.signal.len, self.filter_len);
                    /*println!(" ");
                    for i in 0..self.signal_len{
                        print!("signal_len_after_conv[{}] {} ", i, self.signal[i]);
                    }*/

                    for i in 0..self.contains_len{
                        self.signal_image[i].re=self.signal_image[i].re-self.signal[i].re;
                        self.signal_image[i].im = 0.0;
                    }
                }
                else{
                    self.continue_status = false;
                }
            }
            else {
                self.continue_status = false;
            }
        }
        else if sum>self.min_freq{
            self.iters_counter = self.iters_counter+1;

            for i in 0..self.signal.len{
                self.signal[i].im = 0.0;
            }

            let new_freq:f32 = simple_resampling(self.signal.get_slice(),
                                                 &self.inst_freq[0..self.contains_len],
                                                 &mut self.signal_image,
                                                 &mut self.new_freq_conv,
                                                 &mut self.new_freq_conv_image,
                                                 self.signal_len);

            self.signal_image_len = self.signal_image.len();
            self.resize_after_resampled();

            if new_freq < self.min_freq{
                self.continue_status = false;
            }
            self.signal.set_signal(&self.signal_image);
            self.filter_len = (self.sample_freq * (self.contains_len as f32) / (self.signal_len as f32) /new_freq*self.filter_x) as usize;
            if self.filter_len>self.contains_len{
                self.filter_len = self.contains_len;
            }
            self.filter.resize(self.signal.len*2, Complex{re: 0.0, im: 0.0});

            generate_simple_mask(self.filter_len, self.filter_pow, &mut self.filter[0..self.filter_len]);

            self.signal.conv(&mut self.filter, self.signal.len, self.filter_len);


            for i in 0..self.contains_len{
                self.signal_image[i].re=self.signal_image[i].re-self.signal[i].re;
                self.signal_image[i].im = 0.0;
            }
        }
        else{
            self.continue_status = false;
        }
    }
}