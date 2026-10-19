// root module declaring sub-systems for the biological simulation
pub mod organelle;
pub mod metabolism;
pub mod signaling;

pub use organelle::*;
pub use metabolism::*;
pub use signaling::*;

pub struct whole_cell {
    pub id: u32,
    pub nucleus: nucleus_core,
    pub power_plant: mitochondrion_unit,
    pub factory: ribosome_factory,
    pub fluid: cytoplasm_matrix,
}

impl whole_cell {
    pub fn initialize(id: u32) -> self {
        self {
            id,
            nucleus: nucleus_core::boot(id),
            power_plant: mitochondrion_unit::setup(),
            factory: ribosome_factory::build(),
            fluid: cytoplasm_matrix::create(),
        }
    }

    pub fn execute_cycle(&mut self) {
        self.power_plant.generate_atp(&mut self.fluid);
        self.factory.synthesize(&mut self.fluid);
        self.nucleus.check_dna(&mut self.fluid);
    }
}
