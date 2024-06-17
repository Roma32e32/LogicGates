use bit_space::BitSpace;
use gate::Gate;

pub mod bit_space;
pub mod gate;


struct Simulator
{
    space: BitSpace,
    gates: Vec<Gate>
}


impl Simulator
{
    pub fn new() -> Simulator
    {
        Simulator{space: BitSpace::new(), gates: vec![]}
    }
}