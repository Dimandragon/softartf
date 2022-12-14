use std::default::Default;
use std::ops::{Index, IndexMut};
use num_traits::zero;

pub struct CycleBuffer<T>
    where T: Copy + num_traits::Zero
{
    pub data: Vec<T>,
    pub zero: usize,
    pub len: usize,
}

impl<T> CycleBuffer<T>
    where T: Copy + num_traits::Zero + Default{
    pub fn new() -> Self {
        CycleBuffer {
            data: Vec::new(),
            zero: 0,
            len: 0,
        }
    }

    pub fn new_with_len(size: usize) -> Self {
        CycleBuffer{
            data: vec![zero(); size*2],
            len: size,
            zero: 0,
        }
    }

    pub fn push_element(&mut self, value: T)
        where T: Copy {
        if self.zero >= self.len {
            self.zero = 0;
        } else {
            self.zero = self.zero + 1;
        }
        assert!(self.zero+self.len>0);
        self.data[self.zero + self.len-1] = value;
        if self.zero > 1 {
            self.data[self.zero - 2] = value;
        }
    }

    pub fn push_slice(&mut self, value: &[T])
        where T: Copy {
        let temp: usize;
        if value.len() > self.len {
            temp = self.len;
            self.zero = 0;
            for i in 0..self.len {
                self.push_element(value[value.len()-temp+i]);
            }
        } else {
            temp = value.len();
            for i in 0..temp {
                self.push_element(value[i]);
            }
        }
    }

    pub fn resize(&mut self, new_size: usize){
        let zero_value:T = zero();
        self.data.resize_with(new_size*2, Default::default);
        self.len = new_size;
        for i in 0..self.data.len(){
            self.data[i] = zero_value;
        }
    }

    pub fn len(&self) -> usize
    {
        self.len
    }
}

impl<T> Index<usize> for CycleBuffer<T>
    where T: Copy + num_traits::Zero{
    type Output = T;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.len);
        & self.data[index + self.zero]
    }
}

impl <T> IndexMut<usize> for CycleBuffer<T>
    where T: Copy + num_traits::Zero {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert! (index<self.len);
        &mut self.data[index + self.zero]
    }
}