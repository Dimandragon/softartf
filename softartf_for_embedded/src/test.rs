//use core::slice::SlicePattern;

use std::collections::hash_map;

use crate::test_data::*;
use crate::FrSqIF;
use softartf_enums::InstFreqTypeForResampling;
use softartf_enums::InstAmplType;
use rustfft::FftPlanner;
use num_complex::Complex;
//use crate::FIF;

/*
#[test]
fn cycle_buffer_work() {
    let mut cycle_buffer = CycleBuffer::new_with_len(10);
    for i in 0..50 {
        cycle_buffer.push_element(i);
        for j in 0..cycle_buffer.len() {
            if i + j >= cycle_buffer.len() {
                assert!((cycle_buffer[j] == i + 1 + j - cycle_buffer.len()));
            } else {
                assert!((cycle_buffer[j] == 0));
            }
        }
    }
    cycle_buffer.resize(15);
    for i in 0..50 {
        cycle_buffer.push_element(i);
        for j in 0..cycle_buffer.len() {
            if i + j >= cycle_buffer.len() {
            } else {
                assert!((cycle_buffer[j] == 0));
            }
        }
    }
    cycle_buffer.resize(8);
    for i in 0..50 {
        cycle_buffer.push_element(i);
        for j in 0..cycle_buffer.len() {
            if i + j >= cycle_buffer.len() {
                assert!((cycle_buffer[j] == i + 1 + j - cycle_buffer.len()));
            } else {
                assert!((cycle_buffer[j] == 0));
            }
        }
    }
}

#[test]
fn ht_work(){
    let mut test_data_vec: Vec<Complex<f32>> = get_test_data_complex();
    let mut planner = FftPlanner::new();
    ht(& mut test_data_vec[0..5000], &mut planner);
    let answers_vec = get_ht_test_result_complex();
    for i in 0..test_data_vec.len(){
        assert!((test_data_vec[i]==answers_vec[i]));
    }
}

#[test]
fn inst_freq_work(){
    let mut test_data_vec: Vec<Complex<f32>> = get_test_data_complex();
    let mut test_out_data_vec: Vec<f32> = vec![0.0; test_data_vec.len()];
    let mut planner = FftPlanner::new();
    let len = test_out_data_vec.len();
    inst_freq(&mut test_data_vec[0..len], &mut test_out_data_vec[0..len], 1000.0, &mut planner);
    let answer_vec = get_inst_freq_test_data();
    assert!((answer_vec.len()==test_out_data_vec.len()));
    for i in 0..answer_vec.len(){
        assert!((answer_vec[i]==test_out_data_vec[i]));
    }
}

#[test]
fn inst_freq_extremums_work(){
    let mut test_data_vec: Vec<f32> = get_test_data();
    let mut test_out_data_vec: Vec<f32> = vec![0.0; test_data_vec.len()];
    let mut extremums = Vec::new();
    inst_freq_extremums(&mut test_data_vec, &mut test_out_data_vec, 5000.0, &mut extremums);
    single_plot("inst_freq_extremums.svg", &test_out_data_vec, 100.0, 5.0);
    /*for i in 0..test_out_data_vec.len(){
        print!("{} ", test_out_data_vec[i]);
    }*/
}

#[test]
fn inst_freq_extremums_averaging_work(){
    let mut test_data_vec: Vec<f32> = get_test_data();
    let mut test_out_data_vec: Vec<f32> = vec![0.0; test_data_vec.len()];
    let mut extremums = Vec::new();
    let mut freq_x = Vec::new();
    let mut freq_y = Vec::new();
    inst_freq_extremums_averaging(&mut test_data_vec, &mut test_out_data_vec, 5000.0, &mut extremums, &mut freq_x, &mut freq_y);
    /*for i in 0..test_out_data_vec.len(){
        print!("{} ", test_out_data_vec[i]);
    }*/
    single_plot("inst_freq_extremums_averaging.svg", &test_out_data_vec, 100.0, 5.0);

}

#[test]
fn inst_freq_extremums_complex_work(){
    let mut test_data_vec: Vec<Complex<f32>> = get_test_data_complex();
    let mut test_out_data_vec: Vec<f32> = vec![0.0; test_data_vec.len()];
    let mut extremums = Vec::new();
    inst_freq_extremums_complex(&mut test_data_vec, &mut test_out_data_vec, 5000.0, &mut extremums);
    single_plot("inst_freq_extremums_complex.svg", &test_out_data_vec, 100.0, 5.0);
    /*for i in 0..test_out_data_vec.len(){
        print!("{} ", test_out_data_vec[i]);
    }*/
}

#[test]
fn inst_freq_extremums_averaging_complex_work(){
    let mut test_data_vec: Vec<Complex<f32>> = get_test_data_complex();
    let mut test_out_data_vec: Vec<f32> = vec![0.0; test_data_vec.len()];
    let mut extremums = Vec::new();
    let mut freq_x = Vec::new();
    let mut freq_y = Vec::new();
    inst_freq_extremums_averaging_complex(&mut test_data_vec, &mut test_out_data_vec, 5000.0, &mut extremums, &mut freq_x, &mut freq_y);
    /*for i in 0..test_out_data_vec.len(){
        print!("{} ", test_out_data_vec[i]);
    }*/
    single_plot("inst_freq_extremums_complex_averaging.svg", &test_out_data_vec, 100.0, 5.0);
}

#[test]
/*fn resampling_work(){
    let mut planner = FftPlanner::new();
    let test_data_vec = get_test_data_complex();
    let freq_vec = get_inst_freq_test_data();
    let mut res: Vec<f32> = vec![1.0; 5000];
    let mut test_out: Vec<Complex<f32>> = Vec::with_capacity(test_data_vec.len());
    let mut res_image: Vec<f32> = Vec::with_capacity(test_data_vec.len());
    let freq_conv = simple_resampling(&test_data_vec[0..5000], &freq_vec[0..5000], &mut test_out, &mut res, &mut res_image);
    let mut inst_freq_out: Vec<f32> = vec![0.0; test_out.len()];
    let temp_len = test_out.len();
    inst_freq(&mut test_out[0..temp_len], &mut inst_freq_out[0..temp_len], 1000.0*freq_conv, &mut planner);
    let answer_res_vec = get_resampled_test_data();
    assert!((answer_res_vec.len()==test_out.len()));
    for i in 0..answer_res_vec.len(){
        assert!((answer_res_vec[i]==test_out[i]));
    }
    let mut back_res_test_vec: Vec<Complex<f32>> = Vec::with_capacity(test_data_vec.len());
    let answer_back_res_test_vec: Vec<Complex<f32>> = get_back_resampled_test_data();
    let temp_res_len = test_out.len();
    back_resampling(&test_out[0..temp_res_len], &mut back_res_test_vec, &res);
    assert!((answer_back_res_test_vec.len()==back_res_test_vec.len()));
    for i in 0..answer_back_res_test_vec.len(){
        assert!((answer_back_res_test_vec[i]==back_res_test_vec[i]));
    }
}*/

#[test]
fn averaging_work(){
    let mut test_data_vec: Vec<f32> = get_test_data();
    let window_len: usize = 500;
    let mut buffer: CycleBuffer<f32> = CycleBuffer::new_with_len(500);
    single_plot("before_averaging.svg", &test_data_vec, 100.0, 1.0);
    let len = test_data_vec.len();
    averaging(&mut test_data_vec[0..len], window_len, &mut buffer);
    single_plot("after_averaging.svg", &test_data_vec, 100.0, 1.0);
    let answers_averaging_vec: Vec<f32> = get_averaging_test_data();
    assert!((answers_averaging_vec.len()==test_data_vec.len()));
    for i in 0..test_data_vec.len() {
        assert!((answers_averaging_vec[i]==test_data_vec[i]));
    }
}

#[test]
fn generate_simple_mask_work(){
    let answers_vec: Vec<Vec<Complex<f32>>> = get_filter_answers();
    let mut buffer = vec![Complex {re: 0.0, im: 0.0}; 3];
    generate_simple_mask(3, 1.0, &mut buffer[0..3]);
    for i in 0..3{
        assert!((answers_vec[0][i]==buffer[i]));
    }
    buffer = vec![Complex {re: 0.0, im: 0.0}; 6];
    generate_simple_mask(6, 2.0, &mut buffer[0..6]);
    for i in 0..6{
        assert!((answers_vec[1][i]==buffer[i]));
    }
    buffer = vec![Complex {re: 0.0, im: 0.0}; 10];
    generate_simple_mask(10, 0.1,&mut buffer[0..10]);
    for i in 0..10{
        assert!((answers_vec[2][i]==buffer[i]));
    }
    buffer = vec![Complex {re: 0.0, im: 0.0}; 500];
    generate_simple_mask(500, 0.4, &mut buffer[0..500]);
    for i in 0..500{
        assert!((answers_vec[3][i]==buffer[i]));
    }
}

#[test]
fn my_conv_work(){
    let mut filters_vec: Vec<Vec<Complex<f32>>> = get_filter_answers();
    let mut data: Vec<Complex<f32>> = get_test_data_complex();
    let mut planner = FftPlanner::new();
    let mut copy_buffer_vec: Vec<Complex<f32>> = vec![Complex {re: 0.0, im: 0.0}; 250];
    let vec_conv_answers = get_my_conv_answers();
    for _i in 0..250{
        data.push(Complex{re: 0.0, im: 0.0});
    }
    complex_single_plot("data.svg", &data, 100.0, 1.0);
    for i in 0..filters_vec.len(){
        let len = filters_vec[i].len();
        let mut sum_filter: Complex<f32> = Complex{re: 0.0, im: 0.0};
        for j in 0..len{
            sum_filter = sum_filter + filters_vec[i][j];
        }
        while filters_vec[i].len()<5000+len/2 {
            filters_vec[i].push(Complex{re: 0.0, im: 0.0});
        }
        let len2 = filters_vec[i].len();
        my_conv(&mut data[0..5000+len/2], &mut filters_vec[i][0..len2], &mut planner, len, 5000, &mut copy_buffer_vec[0..len/2]);
        let mut temp_vec = Vec::new();
        for i in 0..5000{
            temp_vec.push(data[i]);
        }
        for j in 0..5000{
            assert!((data[j]==vec_conv_answers[i][j]));
        }
        let mut name: String = String::new();
        name.push_str("conv");
        name.push_str(& format!("{}", i));
        name.push_str(".svg");
        complex_single_plot(name, &temp_vec, 100.0, 1.0);
    }
}

*/
#[test]
fn padding_conv_test(){
    let mut data: Vec<Complex<f32>> = get_test_data_complex();
    let planner = FftPlanner::new();
    let mut conv = crate::structs::PaddedSignalComplex::PaddedSignalComplex::new(crate::math::ffts::rustff_fft::rustfft_fft , crate::math::ffts::rustff_fft::rustfft_ifft);
    let mut filters_vec: Vec<Vec<Complex<f32>>> = get_filter_answers();
    let mut etalon_conv = softartf::structs::PaddedSignalComplex::PaddedSignalComplex::new(planner);
    let mut filters_vec_copy = filters_vec.clone();
    conv.set_signal(&data);
    etalon_conv.set_signal(&data);

    for i in 0..filters_vec.len(){
        let temp = filters_vec[i].len();
        conv.conv(&mut filters_vec[i], 5000, temp);
        etalon_conv.conv(&mut filters_vec_copy[i], 5000, temp);
        
        assert!(conv.len()==etalon_conv.len);
        for i in 0..conv.len(){
            assert!(conv[i]==etalon_conv[i]);
        }

    }
}

#[test]
fn ht_with_etalon(){
    let mut data: Vec<Complex<f32>> = get_test_data_complex();
    let planner = FftPlanner::new();
    let mut ht = crate::structs::PaddedSignalComplex::PaddedSignalComplex::new(crate::math::ffts::rustff_fft::rustfft_fft , crate::math::ffts::rustff_fft::rustfft_ifft);
    let mut etalon_ht = softartf::structs::PaddedSignalComplex::PaddedSignalComplex::new(planner);
    ht.set_signal(&data);
    etalon_ht.set_signal(&data); 
    ht.ht();
    etalon_ht.ht();
    assert!(ht.len()==etalon_ht.len());
    for i in 0..ht.len(){
        assert!(ht[i]==etalon_ht[i]);
    }
}







#[test]
fn its_work(){
    let mut data = get_test_data();
    let mut frsqif = FrSqIF::new(crate::math::ffts::rustff_fft::rustfft_fft
                                 ,crate::math::ffts::rustff_fft::rustfft_ifft);
    frsqif.default_computing(&mut data[0..5000], 1000.0);
}

#[test]
fn with_etalon_version(){
    let data = get_test_data();
    let mut frsqif = FrSqIF::new(crate::math::ffts::rustff_fft::rustfft_fft
                                 ,crate::math::ffts::rustff_fft::rustfft_ifft);
    let mut frsqif_etalon = softartf::FrSqIF::new(FftPlanner::<f32>::new());

    frsqif_etalon.set_signal(data.as_slice());
    frsqif_etalon.set_filter_pow(1.0);
    frsqif_etalon.set_filter_x(0.9);
    frsqif_etalon.set_inst_freq_type_for_resampling(InstFreqTypeForResampling::ExtremumsAveraging);
    frsqif_etalon.set_round(Some(0.01));
    frsqif_etalon.set_freq_detail_hz(1.0);
    frsqif_etalon.set_time_kvant_samples(10);
    frsqif_etalon.set_sample_freq(1000.0);
    frsqif_etalon.config_tf_frame_map();
    frsqif_etalon.set_inst_freq_type(InstFreqTypeForResampling::ExtremumsAveraging);
    frsqif_etalon.set_inst_ampl_type(InstAmplType::SimpleAveraging);
    frsqif_etalon.set_tf_plot_log_base(Some(2.0));
    frsqif_etalon.set_tf_plot_pow_base(Some(1.0));
    frsqif_etalon.set_imfs_frame_detail_plotting(true);
    frsqif_etalon.set_min_freq(5.0);
    frsqif_etalon.config_imf_buffer();
    frsqif_etalon.set_avg_len_mn(0.01);
    frsqif_etalon.set_max_iters(15);
    frsqif_etalon.set_data_plots(false);
    frsqif_etalon.set_frsqif_computing_plots(false);
    frsqif_etalon.compute_frsqif();


    frsqif.set_signal(data.as_slice());
    frsqif.set_filter_pow(1.0);
    frsqif.set_filter_x(0.9);
    frsqif.set_inst_freq_type_for_resampling(InstFreqTypeForResampling::ExtremumsAveraging);
    frsqif.set_round(Some(0.01));
    frsqif.set_freq_detail_hz(1.0);
    frsqif.set_time_kvant_samples(10);
    frsqif.set_sample_freq(1000.0);
    frsqif.config_tf_frame_map();
    frsqif.set_inst_freq_type(InstFreqTypeForResampling::ExtremumsAveraging);
    frsqif.set_inst_ampl_type(InstAmplType::SimpleAveraging);
    frsqif.set_min_freq(5.0);
    frsqif.config_imf_buffer();
    frsqif.set_avg_len_mn(0.01);
    frsqif.set_max_iters(15);
    frsqif.compute_frsqif();

    //print!("{} {}", frsqif.tf_frame_map.as_ref().unwrap().len(), frsqif_etalon.tf_frame_map.as_ref().unwrap().len());

    assert!((frsqif.tf_frame_map.as_ref().unwrap().len()== frsqif_etalon.tf_frame_map.as_ref().unwrap().len()));
    
    for i in 0..frsqif.tf_frame_map.as_ref().unwrap().len(){
        frsqif.tf_frame_map.as_ref().unwrap()[i].iter().for_each(|(&key, &value)| {
            assert!(frsqif_etalon.tf_frame_map.as_ref().unwrap()[i].get(&key).is_some());
            assert!((*(frsqif_etalon.tf_frame_map.as_ref().unwrap()[i].get(&key).unwrap())==value));
        });
    
        /*frsqif_etalon.tf_frame_map.as_ref().unwrap()[i].iter().for_each(|(&key, &value)| {
            assert!(frsqif.tf_frame_map.as_ref().unwrap()[i].get(&key).is_some());
            assert!((*frsqif.tf_frame_map.as_ref().unwrap()[i].get(&key).unwrap()==value));
        });*/
    }

}

/*
#[test]
fn fif_work(){
    let mut data = get_test_data();
    let planner = FftPlanner::new();
    let mut fif = FIF::new(planner);
    fif.default_computing(&mut data[0..5000], 1000.0);
}

*/