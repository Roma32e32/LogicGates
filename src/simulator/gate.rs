
pub enum GateClass
{
    Buffer, Or, And, Xor,
    Not, NOr, NAnd, NXor
}


pub struct Gate
{
    pub class: GateClass,
    pub in1: usize,
    pub in2: usize,
    pub out: usize
}


impl Gate
{
    pub fn new(class: GateClass, in1: usize, in2: usize, out: usize) -> Gate
    {
        Gate { class, in1, in2, out }
    }
}