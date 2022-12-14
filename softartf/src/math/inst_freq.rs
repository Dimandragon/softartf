use num_traits::abs;
use rustfft::{num_complex::Complex};
use crate::math::instrumental_math::ht;


pub fn inst_freq(arr: &mut [Complex<f32>], out: &mut [f32], sample_freq: f32, planner: &mut rustfft::FftPlanner<f32>) {
    ht(arr, planner);
    out[0] = num_traits::float::Float::abs(num_traits::float::Float::atan(arr[1].im/arr[1].re)-num_traits::float::Float::atan(arr[0].im/arr[0].re));
    for i in 1..arr.len(){
        out[i] = num_traits::float::Float::abs(num_traits::float::Float::atan(arr[i].im/arr[i].re)-num_traits::float::Float::atan(arr[i-1].im/arr[i-1].re));
    }
    freq_normalisation(out, sample_freq);
}

pub fn inst_freq_ampl(arr: &mut [Complex<f32>], out_freq: &mut [f32], out_ampl: &mut [f32], sample_freq: f32, planner: &mut rustfft::FftPlanner<f32>){
    ht(arr, planner);
    out_freq[0] = num_traits::identities::Zero::zero();
    out_ampl[0] = num_traits::float::Float::sqrt(arr[0].re.powi(2) + arr[0].im.powi(2));
    for i in 1..arr.len(){
        out_freq[i] = num_traits::float::Float::abs(num_traits::float::Float::atan(arr[i].im/arr[i].re)-num_traits::float::Float::atan(arr[i-1].im/arr[i-1].re));
        out_ampl[i] = num_traits::float::Float::sqrt(arr[i].re.powi(2) + arr[i].im.powi(2));
    }
    freq_normalisation(out_freq, sample_freq);
}

pub fn inst_ampl(arr: &mut [Complex<f32>], out_ampl: &mut [f32], planner: &mut rustfft::FftPlanner<f32>){
    ht(arr, planner);
    out_ampl[0] = num_traits::float::Float::sqrt(arr[0].re.powi(2) + arr[0].im.powi(2));
    for i in 1..arr.len(){
        out_ampl[i] = num_traits::float::Float::sqrt(arr[i].re.powi(2) + arr[i].im.powi(2));
    }
}

pub fn freq_normalisation(arr: &mut [f32], sample_freq: f32){
    let pi: f32 = num_traits::float::FloatConst::PI();
    for i in 0..arr.len(){
        arr[i] = arr[i]*sample_freq/(pi*2.0);
    }
}

pub fn inst_freq_extremums(arr: &[f32], out_freq: &mut[f32], sample_freq: f32,
                           extremums_vec: &mut Vec<usize>, kvant: f32){
    extremums_vec.clear();
    extremums_vec.push(0);
    for i in 1..arr.len()-1{
        if (arr[i]>arr[i-1] && arr[i]>=arr[i+1]) || (arr[i]>=arr[i-1] && arr[i]>arr[i+1]) ||
            (arr[i]<arr[i-1] && arr[i]<=arr[i+1]) || (arr[i]<=arr[i-1] && arr[i]<arr[i+1]){
            if extremums_vec.len()>1{
                if (num_traits::float::Float::abs(arr[extremums_vec[extremums_vec.len()-2]]-
                    arr[extremums_vec[extremums_vec.len()-1]])>kvant &&
                    num_traits::float::Float::abs(arr[extremums_vec[extremums_vec.len()-1]]-
                        arr[i])>kvant) == false{
                    let index = extremums_vec.len()-1;
                    extremums_vec[index] = i;
                }
                else {
                    extremums_vec.push(i);
                }
            }
            else {
                extremums_vec.push(i);
            }
        }
    }
    extremums_vec.push(arr.len()-1);
    let mut counter: usize = 0;
    let mut temp: usize = 1;
    let mut temp_old = 0;
    let mut freq = 0.0;
    while counter<arr.len()-1{
        if temp<extremums_vec.len()-1{
            if extremums_vec[temp+1]==counter{
                temp=temp+1;
            }
            if temp!=temp_old{
                temp_old = temp;
                freq = sample_freq / 2.0 / ((extremums_vec[temp+1] - extremums_vec[temp]) as f32);
            }
        }
        else {
            if temp!=temp_old{
                temp_old = temp;
                freq = sample_freq / 2.0 / ((extremums_vec[temp] - extremums_vec[temp-1]) as f32);
            }
        }
        out_freq[counter] = freq;
        counter = counter+1;
    }
}

pub fn inst_ampl_extremums(arr: &[f32], out_ampl: &mut[f32],
                           extremums_vec: &mut Vec<usize>, kvant: f32){
    extremums_vec.clear();
    extremums_vec.push(0);
    for i in 1..arr.len()-1{
        if (arr[i]>arr[i-1] && arr[i]>=arr[i+1]) || (arr[i]>=arr[i-1] && arr[i]>arr[i+1]) ||
            (arr[i]<arr[i-1] && arr[i]<=arr[i+1]) || (arr[i]<=arr[i-1] && arr[i]<arr[i+1]){
            if extremums_vec.len()>1{
                if (num_traits::float::Float::abs(arr[extremums_vec[extremums_vec.len()-2]]-
                    arr[extremums_vec[extremums_vec.len()-1]])>kvant &&
                    num_traits::float::Float::abs(arr[extremums_vec[extremums_vec.len()-1]]-
                        arr[i])>kvant) == false{
                    let index = extremums_vec.len()-1;
                    extremums_vec[index] = i;
                }
                else {
                    extremums_vec.push(i);
                }
            }
            else {
                extremums_vec.push(i);
            }
        }
    }
    extremums_vec.push(arr.len()-1);
    let mut counter: usize = 0;
    let mut temp: usize = 1;
    let mut temp_old = 0;
    let mut ampl = 0.0;
    while counter<arr.len()-1{
        if temp<extremums_vec.len()-1{
            if extremums_vec[temp+1]==counter{
                temp=temp+1;
            }
            if temp!=temp_old{
                temp_old = temp;
                ampl = num_traits::float::Float::abs(arr[extremums_vec[temp+1]] - arr[extremums_vec[temp]]);
            }
        }
        else {
            if temp!=temp_old{
                temp_old = temp;
                ampl = num_traits::float::Float::abs(arr[extremums_vec[temp+1]] - arr[extremums_vec[temp]]);
            }
        }
        out_ampl[counter] = ampl;
        counter = counter+1;
    }
}

pub fn inst_freq_extremums_averaging(arr: &[f32], out_freq: &mut[f32], sample_freq: f32,
                           extremums_vec: &mut Vec<usize>, freq_x: &mut Vec<f32>, freq_y: &mut Vec<f32>, kvant: f32){
    extremums_vec.clear();
    extremums_vec.push(0);
    for i in 1..arr.len()-1{
        if (arr[i]>arr[i-1] && arr[i]>=arr[i+1]) || (arr[i]>=arr[i-1] && arr[i]>arr[i+1]) ||
            (arr[i]<arr[i-1] && arr[i]<=arr[i+1]) || (arr[i]<=arr[i-1] && arr[i]<arr[i+1]){
            if extremums_vec.len()>1{
                if (num_traits::float::Float::abs(arr[extremums_vec[extremums_vec.len()-2]]-
                    arr[extremums_vec[extremums_vec.len()-1]])>kvant &&
                    num_traits::float::Float::abs(arr[extremums_vec[extremums_vec.len()-1]]-
                        arr[i])>kvant) == false{
                    let index = extremums_vec.len()-1;
                    extremums_vec[index] = i;
                }
                else {
                    extremums_vec.push(i);
                }
            }
            else {
                extremums_vec.push(i);
            }
        }
    }
    extremums_vec.push(arr.len()-1);

    freq_x.clear();
    freq_y.clear();
    freq_x.push(0.0);
    freq_y.push(sample_freq / 2.0 / ((extremums_vec[1] - extremums_vec[0]) as f32));
    //println!("freq_y[0] {} ,", sample_freq/ ((self.extremums_vec[1] - self.extremums_vec[0]) as f32));
    for i in 0..extremums_vec.len()-1{
        freq_x.push((extremums_vec[i]+(extremums_vec[i+1]-extremums_vec[i])/2) as f32);
        freq_y.push(sample_freq / 2.0 / ((extremums_vec[i+1] - extremums_vec[i]) as f32));
    }
    freq_x.push((extremums_vec[extremums_vec.len()-2]+(extremums_vec[extremums_vec.len()-1]-extremums_vec[extremums_vec.len()-2])) as f32 /2.0);
    freq_y.push(sample_freq / 2.0 / ((extremums_vec[extremums_vec.len()-1] - extremums_vec[extremums_vec.len()-2]) as f32));
    //println!("freq_y[last] {}", sample_freq / 2.0 / ((self.extremums_vec[self.extremums_vec.len()-1] - self.extremums_vec[self.extremums_vec.len()-2]) as f32));

    let mut counter: usize = 0;
    let mut temp: usize = 0;
    while counter<arr.len()-1{
        if temp==freq_x.len()-1{
            out_freq[counter] = freq_y[temp] as f32;
        }
        else if (freq_x[temp+1]-freq_x[temp])==0.0{
            //out_freq[counter] = ((self.freq_y[temp] * (1.0-(counter as f32 - self.freq_x[temp]))
            //+ self.freq_y[temp+1] * (1.0 - (self.freq_x[temp+1]-counter as f32))) as f32);
            out_freq[counter] = freq_y[temp] as f32;
            if freq_x[temp+1]<=counter as f32{
                temp = temp+1;
            }
        }
        else{
            out_freq[counter] = ((freq_y[temp] * (freq_x[temp+1] - freq_x[temp]-(counter as f32 - freq_x[temp]))
                + freq_y[temp+1] * (freq_x[temp+1] - freq_x[temp] - (freq_x[temp+1]-counter as f32))) as f32) /
                ((freq_x[temp+1]-freq_x[temp]) as f32);
            if freq_x[temp+1]<=counter as f32{
                temp = temp+1;
            }
        }

        //print!("((self.freq_x[temp+1]-self.freq_x[temp]) as f32) {} ", ((self.freq_x[temp+1]-self.freq_x[temp]) as f32));
        counter = counter + 1;
    }
}



pub fn inst_ampl_extremums_averaging(arr: &[f32], out_freq: &mut[f32],
                                     extremums_vec: &mut Vec<usize>, ampl_x: &mut Vec<f32>, ampl_y: &mut Vec<f32>, kvant: f32){
    extremums_vec.clear();
    extremums_vec.push(0);
    for i in 1..arr.len()-1{
        if (arr[i]>arr[i-1] && arr[i]>=arr[i+1]) || (arr[i]>=arr[i-1] && arr[i]>arr[i+1]) ||
            (arr[i]<arr[i-1] && arr[i]<=arr[i+1]) || (arr[i]<=arr[i-1] && arr[i]<arr[i+1]){
            if extremums_vec.len()>1{
                if (num_traits::float::Float::abs(arr[extremums_vec[extremums_vec.len()-2]]-
                    arr[extremums_vec[extremums_vec.len()-1]])>kvant &&
                    num_traits::float::Float::abs(arr[extremums_vec[extremums_vec.len()-1]]-
                        arr[i])>kvant) == false{
                    let index = extremums_vec.len()-1;
                    extremums_vec[index] = i;
                }
                else {
                    extremums_vec.push(i);
                }
            }
            else {
                extremums_vec.push(i);
            }
        }
    }
    extremums_vec.push(arr.len()-1);

    ampl_x.clear();
    ampl_y.clear();
    ampl_x.push(0.0);
    ampl_y.push(abs(arr[extremums_vec[1]] - arr[extremums_vec[0]]));
    //println!("freq_y[0] {} ,", sample_freq/ ((self.extremums_vec[1] - self.extremums_vec[0]) as f32));
    for i in 0..extremums_vec.len()-1{
        ampl_x.push((extremums_vec[i]+(extremums_vec[i+1]-extremums_vec[i])/2) as f32);
        ampl_y.push(abs(arr[extremums_vec[i+1]] - arr[extremums_vec[i]]));
    }
    ampl_x.push((extremums_vec[extremums_vec.len()-2]+(extremums_vec[extremums_vec.len()-1]-extremums_vec[extremums_vec.len()-2])) as f32 /2.0);
    ampl_y.push(abs(arr[extremums_vec[extremums_vec.len()-1]] - arr[extremums_vec[extremums_vec.len()-2]]));
    //println!("freq_y[last] {}", sample_freq / 2.0 / ((self.extremums_vec[self.extremums_vec.len()-1] - self.extremums_vec[self.extremums_vec.len()-2]) as f32));

    let mut counter: usize = 0;
    let mut temp: usize = 0;
    while counter<arr.len()-1{
        if temp==ampl_x.len()-1{
            out_freq[counter] = ampl_y[temp] as f32;
        }
        else if (ampl_x[temp+1]-ampl_x[temp])==0.0{
            //out_freq[counter] = ((self.freq_y[temp] * (1.0-(counter as f32 - self.freq_x[temp]))
            //+ self.freq_y[temp+1] * (1.0 - (self.freq_x[temp+1]-counter as f32))) as f32);
            out_freq[counter] = ampl_y[temp] as f32;
            if ampl_x[temp+1]<=counter as f32{
                temp = temp+1;
            }
        }
        else{
            out_freq[counter] = ((ampl_y[temp] * (ampl_x[temp+1] - ampl_x[temp]-(counter as f32 - ampl_x[temp]))
                + ampl_y[temp+1] * (ampl_x[temp+1] - ampl_x[temp] - (ampl_x[temp+1]-counter as f32))) as f32) /
                ((ampl_x[temp+1]-ampl_x[temp]) as f32);
            if ampl_x[temp+1]<=counter as f32{
                temp = temp+1;
            }
        }

        //print!("((self.freq_x[temp+1]-self.freq_x[temp]) as f32) {} ", ((self.freq_x[temp+1]-self.freq_x[temp]) as f32));
        counter = counter + 1;
    }
}


pub fn inst_freq_extremums_complex(arr: &[Complex<f32>], inst_freq: &mut[f32], sample_freq: f32,
                           extremums_vec: &mut Vec<usize>, kvant: f32){
    extremums_vec.clear();
    extremums_vec.push(0);
    for i in 1..arr.len()-1{
        if (arr[i].re>arr[i-1].re && arr[i].re>=arr[i+1].re) || (arr[i].re>=arr[i-1].re && arr[i].re>arr[i+1].re) ||
            (arr[i].re<arr[i-1].re && arr[i].re<=arr[i+1].re) || (arr[i].re<=arr[i-1].re && arr[i].re<arr[i+1].re){
            if extremums_vec.len()>1{
                if (num_traits::float::Float::abs(arr[extremums_vec[extremums_vec.len()-2]].re-
                    arr[extremums_vec[extremums_vec.len()-1]].re)>kvant &&
                    num_traits::float::Float::abs(arr[extremums_vec[extremums_vec.len()-1]].re-
                        arr[i].re)>kvant) == false{
                    let index = extremums_vec.len()-1;
                    extremums_vec[index] = i;
                }
                else {
                    extremums_vec.push(i);
                }
            }
            else {
                extremums_vec.push(i);
            }
        }
    }
    extremums_vec.push(arr.len()-1);
    let mut counter: usize = 0;
    let mut temp: usize = 1;
    let mut temp_old = 0;
    let mut freq = 0.0;
    while counter<arr.len()-1{
        if extremums_vec[temp+1]==counter{
            temp=temp+1;
        }
        if temp!=temp_old{
            temp_old = temp;
            freq = sample_freq / ((extremums_vec[temp+1] - extremums_vec[temp-1]) as f32);
        }
        inst_freq[counter]= freq;
        counter = counter+1;
    }
}

pub fn inst_freq_extremums_averaging_complex(arr: &[Complex<f32>], inst_freq: &mut[f32], sample_freq: f32,
                                     extremums_vec: &mut Vec<usize>, freq_x: &mut Vec<f32>, freq_y: &mut Vec<f32>, kvant: f32){
    extremums_vec.clear();
    extremums_vec.push(0);
    for i in 1..arr.len()-1{
        if (arr[i].re>arr[i-1].re && arr[i].re>=arr[i+1].re) || (arr[i].re>=arr[i-1].re && arr[i].re>arr[i+1].re) ||
            (arr[i].re<arr[i-1].re && arr[i].re<=arr[i+1].re) || (arr[i].re<=arr[i-1].re && arr[i].re<arr[i+1].re){
            if extremums_vec.len()>1{
                if (num_traits::float::Float::abs(arr[extremums_vec[extremums_vec.len()-2]].re-
                    arr[extremums_vec[extremums_vec.len()-1]].re)>kvant &&
                    num_traits::float::Float::abs(arr[extremums_vec[extremums_vec.len()-1]].re-
                        arr[i].re)>kvant) == false{
                    let index = extremums_vec.len()-1;
                    extremums_vec[index] = i;
                }
                else {
                    extremums_vec.push(i);
                }
            }
            else {
                extremums_vec.push(i);
            }
        }
    }
    extremums_vec.push(arr.len()-1);

    freq_x.clear();
    freq_y.clear();
    freq_x.push(0.0);
    freq_y.push(sample_freq / 2.0 / ((extremums_vec[1] - extremums_vec[0]) as f32));
    for i in 0..extremums_vec.len()-1{
        freq_x.push((extremums_vec[i]+(extremums_vec[i+1]-extremums_vec[i])/2) as f32);
        freq_y.push(sample_freq / 2.0 / ((extremums_vec[i+1] - extremums_vec[i]) as f32));
    }
    freq_x.push((extremums_vec[extremums_vec.len()-2]+(extremums_vec[extremums_vec.len()-1]-extremums_vec[extremums_vec.len()-2])) as f32 /2.0);
    freq_y.push(sample_freq / 2.0 / ((extremums_vec[extremums_vec.len()-1] - extremums_vec[extremums_vec.len()-2]) as f32));
    freq_x.push(extremums_vec[extremums_vec.len()-1] as f32);
    freq_y.push(sample_freq / 2.0 / ((extremums_vec[extremums_vec.len()-1] - extremums_vec[extremums_vec.len()-2]) as f32));

    let mut counter: usize = 0;
    let mut temp: usize = 1;
    while counter<arr.len()-1{
        if temp==freq_x.len()-1{
            //print!("{} {} {}", counter, arr.len(), temp);
        }
        inst_freq[counter] = ((freq_y[temp] * (freq_x[temp+1] - freq_x[temp]-(counter as f32 - freq_x[temp]))
            + freq_y[temp+1] * (freq_x[temp+1] - freq_x[temp] - (freq_x[temp+1]-counter as f32))) as f32) /
            ((freq_x[temp+1]-freq_x[temp]) as f32);

        if freq_x[temp+1]<=counter as f32{
            temp = temp+1;
        }

        counter = counter + 1;

    }
}




pub fn inst_ampl_extremums_complex(arr: &[Complex<f32>], out_ampl: &mut[f32],
                                   extremums_vec: &mut Vec<usize>, kvant: f32){
    extremums_vec.clear();
    extremums_vec.push(0);
    for i in 1..arr.len()-1{
        if (arr[i].re>arr[i-1].re && arr[i].re>=arr[i+1].re) || (arr[i].re>=arr[i-1].re && arr[i].re>arr[i+1].re) ||
            (arr[i].re<arr[i-1].re && arr[i].re<=arr[i+1].re) || (arr[i].re<=arr[i-1].re && arr[i].re<arr[i+1].re){
            if extremums_vec.len()>1{
                if (num_traits::float::Float::abs(arr[extremums_vec[extremums_vec.len()-2]].re-
                    arr[extremums_vec[extremums_vec.len()-1]].re)>kvant &&
                    num_traits::float::Float::abs(arr[extremums_vec[extremums_vec.len()-1]].re-
                        arr[i].re)>kvant) == false{
                    let index = extremums_vec.len()-1;
                    extremums_vec[index] = i;
                }
                else {
                    extremums_vec.push(i);
                }
            }
            else {
                extremums_vec.push(i);
            }
        }
    }
    extremums_vec.push(arr.len()-1);
    let mut counter: usize = 0;
    let mut temp: usize = 1;
    let mut temp_old = 0;
    let mut ampl = 0.0;
    while counter<arr.len()-1{
        if temp<extremums_vec.len()-1{
            if extremums_vec[temp+1]==counter{
                temp=temp+1;
            }
            if temp!=temp_old{
                temp_old = temp;
                ampl = num_traits::float::Float::abs(arr[extremums_vec[temp+1]].re - arr[extremums_vec[temp]].re);
            }
        }
        else {
            if temp!=temp_old{
                temp_old = temp;
                ampl = num_traits::float::Float::abs(arr[extremums_vec[temp+1]].re - arr[extremums_vec[temp]].re);
            }
        }
        out_ampl[counter] = ampl;
        counter = counter+1;
    }
}

pub fn inst_ampl_extremums_averaging_complex(arr: &[Complex<f32>], out_freq: &mut[f32],
                                             extremums_vec: &mut Vec<usize>, ampl_x: &mut Vec<f32>, ampl_y: &mut Vec<f32>, kvant: f32){
    extremums_vec.clear();
    extremums_vec.push(0);
    for i in 1..arr.len()-1{
        if (arr[i].re>arr[i-1].re && arr[i].re>=arr[i+1].re) || (arr[i].re>=arr[i-1].re && arr[i].re>arr[i+1].re) ||
            (arr[i].re<arr[i-1].re && arr[i].re<=arr[i+1].re) || (arr[i].re<=arr[i-1].re && arr[i].re<arr[i+1].re){
            if extremums_vec.len()>1{
                if (num_traits::float::Float::abs(arr[extremums_vec[extremums_vec.len()-2]].re-
                    arr[extremums_vec[extremums_vec.len()-1]].re)>kvant &&
                    num_traits::float::Float::abs(arr[extremums_vec[extremums_vec.len()-1]].re-
                        arr[i].re)>kvant) == false{
                    let index = extremums_vec.len()-1;
                    extremums_vec[index] = i;
                }
                else {
                    extremums_vec.push(i);
                }
            }
            else {
                extremums_vec.push(i);
            }
        }
    }
    extremums_vec.push(arr.len()-1);

    ampl_x.clear();
    ampl_y.clear();
    ampl_x.push(0.0);
    ampl_y.push(abs(arr[extremums_vec[1]].re - arr[extremums_vec[0]].re));
    //println!("freq_y[0] {} ,", sample_freq/ ((self.extremums_vec[1] - self.extremums_vec[0]) as f32));
    for i in 0..extremums_vec.len()-1{
        ampl_x.push((extremums_vec[i]+(extremums_vec[i+1]-extremums_vec[i])/2) as f32);
        ampl_y.push(abs(arr[extremums_vec[i+1]].re - arr[extremums_vec[i]].re));
    }
    ampl_x.push((extremums_vec[extremums_vec.len()-2]+(extremums_vec[extremums_vec.len()-1]-extremums_vec[extremums_vec.len()-2])) as f32 /2.0);
    ampl_y.push(abs(arr[extremums_vec[extremums_vec.len()-1]].re - arr[extremums_vec[extremums_vec.len()-2]].re));
    //println!("freq_y[last] {}", sample_freq / 2.0 / ((self.extremums_vec[self.extremums_vec.len()-1] - self.extremums_vec[self.extremums_vec.len()-2]) as f32));

    let mut counter: usize = 0;
    let mut temp: usize = 0;
    while counter<arr.len()-1{
        if temp==ampl_x.len()-1{
            out_freq[counter] = ampl_y[temp] as f32;
        }
        else if (ampl_x[temp+1]-ampl_x[temp])==0.0{
            //out_freq[counter] = ((self.freq_y[temp] * (1.0-(counter as f32 - self.freq_x[temp]))
            //+ self.freq_y[temp+1] * (1.0 - (self.freq_x[temp+1]-counter as f32))) as f32);
            out_freq[counter] = ampl_y[temp] as f32;
            if ampl_x[temp+1]<=counter as f32{
                temp = temp+1;
            }
        }
        else{
            out_freq[counter] = ((ampl_y[temp] * (ampl_x[temp+1] - ampl_x[temp]-(counter as f32 - ampl_x[temp]))
                + ampl_y[temp+1] * (ampl_x[temp+1] - ampl_x[temp] - (ampl_x[temp+1]-counter as f32))) as f32) /
                ((ampl_x[temp+1]-ampl_x[temp]) as f32);
            if ampl_x[temp+1]<=counter as f32{
                temp = temp+1;
            }
        }

        //print!("((self.freq_x[temp+1]-self.freq_x[temp]) as f32) {} ", ((self.freq_x[temp+1]-self.freq_x[temp]) as f32));
        counter = counter + 1;
    }
}

