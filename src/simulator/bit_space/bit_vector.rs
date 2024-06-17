pub fn get_bit(value: u64, index: usize) -> bool
{
    value & 1 << index != 0
}

pub fn on_bit(value: u64, index: usize) -> u64
{
    value | 1 << index
}

pub fn off_bit(value: u64, index: usize) -> u64
{
    value & !(1 << index)
}

pub fn flip_bit(value: u64, index: usize) -> u64
{
    value ^ 1 << index
}


pub fn index_to_address(index: usize) -> (usize, usize)
{
    (index / 64, index % 64)
}

pub fn address_to_index(address: (usize, usize)) -> usize
{
    address.0 * 64 + address.1
}


pub struct BitVector
{
    data: Vec<u64>,
    bits_used: usize,
}


impl BitVector
{
    pub fn new() -> BitVector
    {
        BitVector{data: vec![], bits_used: 0}
    }

    pub fn copy_data_from(self: &mut BitVector, source: &BitVector)
    {
        debug_assert!(self.data.len() == source.data.len());
        
        for i in 0..self.data.len()
        {
            self.data[i] = source.data[i];
        }
    }

    pub fn alloc_bit(self: &mut BitVector) -> usize
    {
        let max_len = self.data.len() * 64;

        if self.bits_used <= max_len
        {
            self.bits_used += 1;
            self.bits_used - 1
        }
        else
        {
            self.data.push(0);
            self.bits_used += 1;
            self.bits_used - 1        
        }
    }

    pub fn len(self: &BitVector) -> usize
    {
        self.bits_used
    }

    pub fn get_bit(self: &BitVector, index: usize) -> bool
    {
        debug_assert!(index < self.bits_used as usize, "self.bits_used = {} || index = {}", self.bits_used, index);
        let addr = index_to_address(index);
        get_bit(self.data[addr.0], addr.1)
    }

    pub fn on_bit(self: &mut BitVector, index: usize)
    {
        debug_assert!(index < self.bits_used as usize, "self.bits_used = {} || index = {}", self.bits_used, index);
        let addr = index_to_address(index);
        self.data[addr.0] = on_bit(self.data[addr.0], addr.1);
    }

    pub fn off_bit(self: &mut BitVector, index: usize)
    {
        debug_assert!(index < self.bits_used as usize, "self.bits_used = {} || index = {}", self.bits_used, index);
        let addr = index_to_address(index);
        self.data[addr.0] = off_bit(self.data[addr.0], addr.1);
    }

    pub fn flip_bit(self: &mut BitVector, index: usize)
    {
        debug_assert!(index < self.bits_used as usize, "self.bits_used = {} || index = {}", self.bits_used, index);
        let addr = index_to_address(index);
        self.data[addr.0] = flip_bit(self.data[addr.0], addr.1);
    }
}