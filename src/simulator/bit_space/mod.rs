pub mod bit_vector;
use bit_vector::{get_bit, on_bit, BitVector};

pub struct BitSpace
{
    result: BitVector, //куда пишутся результаты вычислений
    source: BitVector, //откуда исходные для вычислений
    consts: BitVector //какие биты всегда on
}


impl BitSpace
{
    pub fn new() -> BitSpace
    {
        BitSpace{result: BitVector::new(), source: BitVector::new(), consts: BitVector::new()}
    }

    pub fn alloc_bit(self: &mut BitSpace) -> usize
    {
        self.result.alloc_bit();
        self.source.alloc_bit();
        self.consts.alloc_bit()
    }

    pub fn len(self: &BitSpace) -> usize
    {
        self.result.len()
    }

    pub fn get_bit(self: &BitSpace, index: usize) -> bool
    {
        self.source.get_bit(index)
    }

    pub fn get_bit_const(self: &BitSpace, index: usize) -> bool
    {
        self.consts.get_bit(index)
    }

    pub fn on_bit(self: &mut BitSpace, index: usize)
    {
        self.result.on_bit(index);
    }

    pub fn off_bit(self: &mut BitSpace, index: usize)
    {
        self.result.off_bit(index);
    }

    pub fn flip_bit(self: &mut BitSpace, index: usize)
    {
        self.result.flip_bit(index);
    }

    pub fn on_bit_const(self: &mut BitSpace, index: usize)
    {
        self.consts.on_bit(index);
    }

    pub fn off_bit_const(self: &mut BitSpace, index: usize)
    {
        self.consts.off_bit(index);
    }

    pub fn flip_bit_const(self: &mut BitSpace, index: usize)
    {
        self.consts.flip_bit(index);
    }

    pub fn insert_bits(self: &mut BitSpace, indexes: &[usize], value: u64)
    {   
        debug_assert!(indexes.len() <= 64);
        let mut i: usize = 0;
        for index in indexes
        {
            if get_bit(value, i)
            {
                self.on_bit_const(*index);
            }
            else
            {
                self.off_bit_const(*index);
            }

            i += 1;
        }
    }

    pub fn exsert_bits(self: &BitSpace, indexes: &[usize]) -> u64
    {
        debug_assert!(indexes.len() <= 64);
        let mut output: u64 = 0;
        let mut i = 0;

        for index in indexes
        {
            if self.get_bit(*index)
            {
                output = on_bit(output, i);
            }
            i += 1;
        }
        output
    }

    pub fn update(self: &mut BitSpace)
    {
        self.source.copy_data_from(&self.result);
        self.result.copy_data_from(&self.consts);
    }
}