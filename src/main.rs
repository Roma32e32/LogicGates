mod bit_space;
use bit_space::bit_vector::*;

fn main()
{
    let mut a = BitVector::new();
    a.alloc_bit();

    let mut b = BitVector::new();

    b.copy_data_from(&a);
        
    a.on_bit(0);

}
