pub mod bit_vector;
use bit_vector::BitVector;

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

    pub fn update(self: &mut BitSpace)
    {
        self.source.copy_data_from(&self.result);
        self.result.copy_data_from(&self.consts);
    }
}