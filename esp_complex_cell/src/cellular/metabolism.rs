// manages chemical reactions, substrate pools and thermal regulation
pub struct mitochondrion_unit {
    pub efficiency_ratio: f32,
    pub thermal_load: u32,
}

impl mitochondrion_unit {
    pub fn setup() -> self {
        self {
            efficiency_ratio: 0.94,
            thermal_load: 35,
        }
    }

    pub fn generate_atp(&mut self, matrix: &mut cytoplasm_matrix) {
        if matrix.glucose >= 2 {
            matrix.glucose -= 2;
            let generated = (15.0 * self.efficiency_ratio) as u32;
            matrix.atp_level += generated;
            self.thermal_load = self.thermal_load.saturating_add(1);
        } else {
            self.thermal_load = self.thermal_load.saturating_sub(2);
        }
    }
}

pub struct cytoplasm_matrix {
    pub glucose: u32,
    pub atp_level: u32,
    pub amino_acids: u32,
    pub waste_byproducts: u32,
}

impl cytoplasm_matrix {
    pub fn create() -> self {
        self {
            glucose: 1000,
            atp_level: 500,
            amino_acids: 800,
            waste_byproducts: 0,
        }
    }
}
