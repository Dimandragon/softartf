use std::collections::HashMap;
use rustfft::{FftPlanner, num_complex::Complex};
use std::usize::MAX;
use num_traits::Zero;
use crate::structs::CycleBuffer::CycleBuffer;

pub fn min<T>(data: &[T])->Option<T>
    where T: Copy + num_traits::Zero + PartialOrd{
    if data.len()==0{
        return None;
    }else if data.len()==1{
        return Some(data[0]);
    }
    else{
        let mut minimum: T;
        if data[0]<data[1]{
            minimum = data[0];
        }
        else{
            minimum = data[1];
        }
        for i in 2..data.len(){
            if data[i]<minimum{
                minimum = data[i];
            }
        }
        return Some(minimum);
    }
}

pub fn min_map<K, T>(data: &HashMap<K, T>)->Option<(K, T)>
    where T: Copy + Zero + std::cmp::PartialOrd, K: Copy + Zero + std::cmp::PartialOrd
{
    if data.len()==0{
        return None;
    }
    else{
        let mut minimum_value: T = num_traits::Zero::zero();
        let mut minimum_key: K = num_traits::Zero::zero();
        let mut first = true;
        for (keys, values) in data.iter(){
            if first{
                first = false;
                minimum_value = *values;
                minimum_key = *keys;
            }
            else{
                if *values<minimum_value{
                    minimum_value = *values;
                }
                if *keys<minimum_key{
                    minimum_key = *keys;
                }
            }
        }
        let minimum = (minimum_key, minimum_value);
        return Some(minimum);
    }
}

pub fn max_map<K, T>(data: &HashMap<K, T>)->Option<(K, T)>
    where T: Copy + Zero + std::cmp::PartialOrd, K: Copy + Zero + std::cmp::PartialOrd
{
    if data.len()==0{
        return None;
    }
    else{
        let mut maximum_value: T = num_traits::Zero::zero();
        let mut maximum_key: K = num_traits::Zero::zero();
        let mut first = true;
        for (keys, values) in data.iter(){
            if first{
                first = false;
                maximum_value = *values;
                maximum_key = *keys;
            }
            else{
                if *values>maximum_value{
                    maximum_value = *values;
                }
                if *keys>maximum_key{
                    maximum_key = *keys;
                }
            }
        }
        let maximum = (maximum_key, maximum_value);
        return Some(maximum);
    }
}
/*
pub fn min_vec_map<K, T>(data: &Vec<HashMap<K, T>>)->Option<(K, T)>
    where T: Copy + Zero + std::cmp::PartialOrd, K: Copy + Zero + std::cmp::PartialOrd
{
    if data.len()==0{
        return None;
    }
    else{
        let mut out = min_map(&data[0]);

        for i in 0..data.len(){
            if out.is_none(){
                out = min_map(&data[i]);
            }
            else {
                let mut temp = min_map(&data[i]);
                if temp.is_some(){
                    if temp.unwrap().0 < out.unwrap().0 {
                        out.unwrap().0 = temp.unwrap().0;
                    }
                    if temp.unwrap().1 < out.unwrap().1 {
                        out.unwrap().1 = temp.unwrap().1;
                    }
                }
            }
        }
        out
    }
}

pub fn max_vec_map<K, T>(data: &Vec<HashMap<K, T>>)->Option<(K, T)>
    where T: Copy + Zero + std::cmp::PartialOrd, K: Copy + Zero + std::cmp::PartialOrd
{
    if data.len()==0{
        return None;
    }
    else{
        let mut out = max_map(&data[0]);

        for i in 0..data.len(){
            if out.is_none(){
                out = max_map(&data[i]);
            }
            else {
                let mut temp = max_map(&data[i]);
                if temp.is_some(){
                    if temp.unwrap().0 > out.unwrap().0 {
                        out.unwrap().0 = temp.unwrap().0;
                    }
                    if temp.unwrap().1 > out.unwrap().1 {
                        out.unwrap().1 = temp.unwrap().1;
                    }
                }
            }
        }
        out
    }
}
*/
/*
pub fn max_vec_map<K, T>(data: &Vec<HashMap<K, T>>)->Option<(K, T)>
    where T: Copy + Zero + std::cmp::PartialOrd, K: Copy + Zero + std::cmp::PartialOrd
{
    if data.len()==0{
        return None;
    }
    else{
        let mut maximum: Option<(K, T)> = None;
        for hash_map in data{
            for (&key, &value) in hash_map{
                if maximum.is_none(){
                    maximum = Some((key, value));
                }
                else {
                    if key>maximum.unwrap().0{
                        maximum.unwrap().0 = key;
                    }
                    if value>maximum.unwrap().1{
                        maximum.unwrap().1 = value;
                    }
                }
            }
        }
        return maximum
    }
}

pub fn min_vec_map<K, T>(data: &Vec<HashMap<K, T>>)->Option<(K, T)>
    where T: Copy + Zero + std::cmp::PartialOrd, K: Copy + Zero + std::cmp::PartialOrd
{
    if data.len()==0{
        return None;
    }
    else{
        let mut minimum: Option<(K, T)> = None;
        for hash_map in data{
            for (&key, &value) in hash_map{
                if minimum.is_none(){
                    minimum = Some((key, value));
                }
                else {
                    if key<minimum.unwrap().0{
                        minimum.unwrap().0 = key;
                    }
                    if value<minimum.unwrap().1{
                        minimum.unwrap().1 = value;
                    }
                }
            }
        }
        return minimum
    }
}
*/



pub fn max_vec_map<K, T>(data: &Vec<HashMap<K, T>>)->Option<(K, T)>
    where T: Copy + Zero + std::cmp::PartialOrd, K: Copy + Zero + std::cmp::PartialOrd
{
    if data.len()==0{
        return None;
    }
    else{
        let mut maximum: Option<(K, T)> = None;
        for (x, hash_map) in data.iter().enumerate()
        {
            hash_map.iter().for_each(|(&y, &value)|{
                if maximum.is_none(){
                    maximum = Some((y, value));
                }
                else {
                    if y>maximum.unwrap().0{
                        maximum.as_mut().unwrap().0 = y;
                    }
                    if value>maximum.unwrap().1{
                        maximum.as_mut().unwrap().1 = value;
                    }
                }
            });
        }
        return maximum
    }
}

pub fn min_vec_map<K, T>(data: &Vec<HashMap<K, T>>)->Option<(K, T)>
    where T: Copy + Zero + std::cmp::PartialOrd, K: Copy + Zero + std::cmp::PartialOrd
{
    if data.len()==0{
        return None;
    }
    else{
        let mut minimum: Option<(K, T)> = None;
        for (x, hash_map) in data.iter().enumerate()
        {
            hash_map.iter().for_each(|(&y, &value)|{
                if minimum.is_none(){
                    minimum = Some((y, value));
                }
                else {
                    if y<minimum.unwrap().0{
                        minimum.as_mut().unwrap().0 = y;
                    }
                    if value<minimum.unwrap().1{
                        minimum.as_mut().unwrap().1 = value;
                    }
                }
            });
        }
        return minimum
    }
}



pub fn max<T>(data: &[T])->Option<T>
    where T: Copy + num_traits::Zero + PartialOrd{
    if data.len()==0{
        return None;
    }else if data.len()==1{
        return Some(data[0]);
    }
    else{
        let mut maximum: T;
        if data[0]>data[1]{
            maximum = data[0];
        }
        else{
            maximum = data[1];
        }
        for i in 2..data.len(){
            if data[i]>maximum{
                maximum = data[i];
            }
        }
        return Some(maximum);
    }
}

pub fn min_complex_ignore<T>(data: &[Complex<T>])->Option<T>
    where T: Copy + num_traits::Zero + PartialOrd{
    if data.len()==0{
        return None;
    }else if data.len()==1{
        return Some(data[0].re);
    }
    else{
        let mut minimum: T;
        if data[0].re<data[1].re{
            minimum = data[0].re;
        }
        else{
            minimum = data[1].re;
        }
        for i in 2..data.len(){
            if data[i].re<minimum{
                minimum = data[i].re;
            }
        }
        return Some(minimum);
    }
}

pub fn max_complex_ignore<T>(data: &[Complex<T>])->Option<T>
    where T: Copy + num_traits::Zero + PartialOrd{
    if data.len()==0{
        return None;
    }else if data.len()==1{
        return Some(data[0].re);
    }
    else{
        let mut maximum: T;
        if data[0].re>data[1].re{
            maximum = data[0].re;
        }
        else{
            maximum = data[1].re;
        }
        for i in 2..data.len(){
            if data[i].re>maximum{
                maximum = data[i].re;
            }
        }
        return Some(maximum);
    }
}

pub fn min_complex<T>(data: &[Complex<T>])->Option<T>
    where T: Copy + num_traits::Zero + PartialOrd{
    if data.len()==0{
        return None;
    }else if data.len()==1{
        if data[0].re>data[0].im{
            return Some(data[0].im);
        }
        else{
            return Some(data[0].re);
        }
    }
    else{
        let mut minimum: T;
        if data[0].re>data[0].im{
            minimum = data[0].im;
        }
        else{
            minimum = data[1].re;
        }
        for i in 1..data.len(){
            if data[i].re<minimum{
                minimum = data[i].re;
            }
            if data[i].im<minimum{
                minimum = data[i].im;
            }
        }
        return Some(minimum);
    }
}

pub fn max_complex<T>(data: &[Complex<T>])->Option<T>
    where T: Copy + num_traits::Zero + PartialOrd{
    if data.len()==0{
        return None;
    }else if data.len()==1{
        if data[0].re<data[0].im{
            return Some(data[0].im);
        }
        else{
            return Some(data[0].re);
        }
    }
    else{
        let mut maximum: T;
        if data[0].re<data[0].im{
            maximum = data[0].im;
        }
        else{
            maximum = data[1].re;
        }
        for i in 1..data.len(){
            if data[i].re>maximum{
                maximum = data[i].re;
            }
            if data[i].im>maximum{
                maximum = data[i].im;
            }
        }
        return Some(maximum);
    }
}

pub fn min_double_arr<T>(data: &Vec<&Vec<T>>)->Option<f64>
    where T: Copy + num_traits::Zero + PartialOrd, f64: From<T>{
    if data.len()==0{
        return None;
    }else if data.len()==1{
        if data[0].len()==0{
            return None;
        }
        else{
            return Some(min(data[0].as_slice()).unwrap().into());
        }
    }
    else{
        let mut noon: bool = true;
        let mut minimum: f64 = f64::MAX;
        for _i in 0..data.len(){
            if min(data[0].as_slice()).is_some(){
                if minimum> (min(data[0].as_slice()).unwrap()).into(){
                    minimum = (min(data[0].as_slice()).unwrap()).into();
                    noon = false;
                }
            }
        }
        if noon==true{
            return None
        }
        else{
            return Some(minimum)
        }
    }
}

pub fn max_double_arr<T>(data: &Vec<&Vec<T>>)->Option<f64>
    where T: Copy + num_traits::Zero + PartialOrd, f64: From<T>{
    if data.len()==0{
        return None;
    }else if data.len()==1{
        if data[0].len()==0{
            return None;
        }
        else{
            return Some(max(data[0].as_slice()).unwrap().into());
        }
    }
    else{
        let mut noon: bool = true;
        let mut maximum: f64 = f64::MIN;
        for _i in 0..data.len(){
            if max(data[0].as_slice()).is_some(){
                if maximum<max(data[0].as_slice()).unwrap().into(){
                    maximum = max(data[0].as_slice()).unwrap().into();
                    noon = false;
                }
            }
        }
        if noon==true{
            return None
        }
        else{
            return Some(maximum)
        }
    }
}

pub fn min_double_arr_complex<T>(data: &Vec<&Vec<Complex<T>>>)->Option<f64>
    where T: Copy + num_traits::Zero + PartialOrd, f64: From<T>{
    if data.len()==0{
        return None;
    }else if data.len()==1{
        if data[0].len()==0{
            return None;
        }
        else{
            return Some(min_complex(data[0].as_slice()).unwrap().into());
        }
    }
    else{
        let mut noon: bool = true;
        let mut minimum: f64 = f64::MAX;
        for _i in 0..data.len(){
            if min_complex(data[0].as_slice()).is_some(){
                if minimum> (min_complex(data[0].as_slice()).unwrap()).into(){
                    minimum = (min_complex(data[0].as_slice()).unwrap()).into();
                    noon = false;
                }
            }
        }
        if noon==true{
            return None
        }
        else{
            return Some(minimum)
        }
    }
}

pub fn max_double_arr_complex<T>(data: &Vec<&Vec<Complex<T>>>)->Option<f64>
    where T: Copy + num_traits::Zero + PartialOrd, f64: From<T>{
    if data.len()==0{
        return None;
    }else if data.len()==1{
        if data[0].len()==0{
            return None;
        }
        else{
            return Some(max_complex(data[0].as_slice()).unwrap().into());
        }
    }
    else{
        let mut noon: bool = true;
        let mut maximum: f64 = f64::MIN;
        for _i in 0..data.len(){
            if max_complex(data[0].as_slice()).is_some(){
                if maximum<max_complex(data[0].as_slice()).unwrap().into(){
                    maximum = max_complex(data[0].as_slice()).unwrap().into();
                    noon = false;
                }
            }
        }
        if noon==true{
            return None
        }
        else{
            return Some(maximum)
        }
    }
}

pub fn min_double_arr_complex_ignore<T>(data: &Vec<&Vec<Complex<T>>>)->Option<f64>
    where T: Copy + num_traits::Zero + PartialOrd, f64: From<T>{
    if data.len()==0{
        return None;
    }else if data.len()==1{
        if data[0].len()==0{
            return None;
        }
        else{
            return Some(min_complex_ignore(data[0].as_slice()).unwrap().into());
        }
    }
    else{
        let mut noon: bool = true;
        let mut minimum: f64 = f64::MAX;
        for _i in 0..data.len(){
            if min_complex_ignore(data[0].as_slice()).is_some(){
                if minimum> (min_complex_ignore(data[0].as_slice()).unwrap()).into(){
                    minimum = (min_complex_ignore(data[0].as_slice()).unwrap()).into();
                    noon = false;
                }
            }
        }
        if noon==true{
            return None
        }
        else{
            return Some(minimum)
        }
    }
}

pub fn max_double_arr_complex_ignore<T>(data: &Vec<&Vec<Complex<T>>>)->Option<f64>
    where T: Copy + num_traits::Zero + PartialOrd, f64: From<T>{
    if data.len()==0{
        return None;
    }else if data.len()==1{
        if data[0].len()==0{
            return None;
        }
        else{
            return Some(max_complex_ignore(data[0].as_slice()).unwrap().into());
        }
    }
    else{
        let mut noon: bool = true;
        let mut maximum: f64 = f64::MIN;
        for _i in 0..data.len(){
            if max_complex_ignore(data[0].as_slice()).is_some(){
                if maximum<max_complex_ignore(data[0].as_slice()).unwrap().into(){
                    maximum = max_complex_ignore(data[0].as_slice()).unwrap().into();
                    noon = false;
                }
            }
        }
        if noon==true{
            return None
        }
        else{
            return Some(maximum)
        }
    }
}

pub fn max_len<T>(data: &Vec<&Vec<T>>)->Option<usize>{
    if data.len()==0{
        return None;
    }
    else{
        let mut max: usize = 0;
        for i in 0..data.len(){
            if data[i].len()>max{
                max = data[i].len();
            }
        }
        return Some(max);
    }
}

pub fn min_len<T>(data: &Vec<&Vec<T>>)->Option<usize>{
    if data.len()==0{
        return None;
    }
    else{
        let mut min: usize = MAX;
        for i in 0..data.len(){
            if data[i].len()<min{
                min = data[i].len();
            }
        }
        return Some(min);
    }
}

pub fn sqr_avg_diff(arr: &[f32])->f32{
    let mut avg: f64 = 0.0;
    for i in 1..arr.len(){
        avg = avg + (((arr[i]-arr[i-1])*(arr[i]-arr[i-1])) as f64);
    }
    avg = avg / (arr.len()-1) as f64;
    avg = avg.sqrt();
    let out = avg as f32;
    out
}

pub fn averaging(arr: &mut [f32], window_len: usize, buffer: &mut CycleBuffer<f32>){
    if buffer.len!=window_len{
        buffer.resize(window_len);
    }
    let mut sum: f32 = 0.0;
    for i in 0..window_len{
        if i<window_len/2{
            buffer.push_element(arr[window_len/2-i]);
            sum = sum + arr[window_len/2-i]/window_len as f32;
        }
        else{
            buffer.push_element(arr[i]);
            sum = sum + arr[i]/window_len as f32;
        }
    }
    for i in 0..arr.len()-window_len/2-window_len%2{
        arr[i] = sum;
        sum = sum - buffer[0]/window_len as f32;
        buffer.push_element(arr[i+window_len/2+window_len%2]);
        sum = sum + arr[i+window_len/2+window_len%2]/window_len as f32;
    }
    for i in arr.len()-window_len/2-window_len%2..arr.len(){
        arr[i] = sum;
        sum = sum - buffer[0]/window_len as f32;
        buffer.push_element(arr[arr.len()-i+arr.len()-window_len/2-window_len%2-1]);
        sum = sum + arr[arr.len()-i+arr.len()-window_len/2-window_len%2-1]/window_len as f32;
    }
    arr[arr.len()-1] = sum;
}

pub fn ht (buffer: &mut [Complex<f32>], planner: &mut FftPlanner<f32>) {
    let fft = planner.plan_fft_forward(buffer.len());
    let ifft = planner.plan_fft_inverse(buffer.len());
    for i in 0..buffer.len(){
        buffer[i].im = num_traits::identities::Zero::zero();
    }
    fft.process(buffer);
    for i in 0..buffer.len()/2 + buffer.len() % 2
    {
        buffer[i].re=0.0;
        buffer[i].im=0.0;
    }
    for i in buffer.len()/2 + buffer.len() % 2 .. buffer.len()
    {
        buffer[i].re=buffer[i].re*2.0/(buffer.len() as f32);
        buffer[i].im=buffer[i].im*2.0/(buffer.len() as f32);
    }
    ifft.process(buffer);
}

pub fn pow_sum (a: usize, pow: f32) ->f32 {
    let mut result: f32 = num_traits::identities::Zero::zero();
    for i in 0..a
    {
        let temp: f32 = (i+1) as f32;
        result = result + temp.powf(pow);
    }
    result
}

