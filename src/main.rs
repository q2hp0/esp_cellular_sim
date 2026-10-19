// local desktop test runner for complex cell simulation

use std::thread;
use std::time::Duration;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CellLifecycleState {
    Dormant,
    Proliferating,
    MetabolicallyActive,
    OxidativelyStressed,
    Senescent,
    Apoptotic,
}

pub struct DnaHelixStructure {
    pub sequence_slots: [u8; 64],
    pub mutation_count: u32,
    pub telomere_length: u32,
    pub methylation_status: bool,
    pub damage_repair_efficiency: f32,
}

impl DnaHelixStructure {
    pub fn initialize(seed_val: u32) -> Self {
        let mut slots = [0u8; 64];
        for i in 0..64 {
            slots[i] = ((seed_val.wrapping_mul(i as u32 + 1)) % 256) as u8;
        }
        Self {
            sequence_slots: slots,
            mutation_count: 0,
            telomere_length: 1000,
            methylation_status: false,
            damage_repair_efficiency: 0.99,
        }
    }

    pub fn mutate_sequence(&mut self, factor: u8) {
        let target_idx = (factor as usize) % 64;
        self.sequence_slots[target_idx] = self.sequence_slots[target_idx].wrapping_add(factor);
        self.mutation_count = self.mutation_count.wrapping_add(1);
        if self.telomere_length > 0 {
            self.telomere_length -= 1;
        }
    }
}

pub struct NuclearMembranePore {
    pub active_transport_capacity: u32,
    pub protein_shuttle_rate: u32,
    pub is_gated_open: bool,
}

impl NuclearMembranePore {
    pub fn setup() -> Self {
        Self {
            active_transport_capacity: 100,
            protein_shuttle_rate: 25,
            is_gated_open: true,
        }
    }

    pub fn regulate_shuttle(&mut self, cellular_energy: &mut u32) {
        if *cellular_energy >= 15 {
            *cellular_energy -= 15;
            self.is_gated_open = true;
            self.protein_shuttle_rate = 40;
        } else {
            self.is_gated_open = false;
            self.protein_shuttle_rate = 5;
        }
    }
}

pub struct CellNucleus {
    pub dna: DnaHelixStructure,
    pub pore: NuclearMembranePore,
    pub transcription_buffer: u32,
    pub replication_fork_active: bool,
}

impl CellNucleus {
    pub fn build(seed: u32) -> Self {
        Self {
            dna: DnaHelixStructure::initialize(seed),
            pore: NuclearMembranePore::setup(),
            transcription_buffer: 50,
            replication_fork_active: false,
        }
    }

    pub fn execute_transcription(&mut self, atp_pool: &mut u32, amino_pool: &mut u32) {
        self.pore.regulate_shuttle(atp_pool);
        if self.pore.is_gated_open && *amino_pool >= 10 {
            *amino_pool -= 10;
            self.transcription_buffer = self.transcription_buffer.saturating_add(15);
        } else {
            self.transcription_buffer = self.transcription_buffer.saturating_sub(5);
        }
    }
}

pub struct MitochondrionUnit {
    pub unit_id: u32,
    pub membrane_potential: f32,
    pub oxidative_efficiency: f32,
    pub thermal_index: u32,
    pub cytochrome_c_leak: bool,
}

impl MitochondrionUnit {
    pub fn new(id: u32) -> Self {
        Self {
            unit_id: id,
            membrane_potential: -140.0,
            oxidative_efficiency: 0.95,
            thermal_index: 37,
            cytochrome_c_leak: false,
        }
    }

    pub fn perform_oxidative_phosphorylation(&mut self, glucose_pool: &mut u32, atp_pool: &mut u32, oxygen_stress: &mut u32) {
        if *glucose_pool >= 4 {
            *glucose_pool -= 4;
            let yield_atp = (32.0 * self.oxidative_efficiency) as u32;
            *atp_pool += yield_atp;
            self.thermal_index = self.thermal_index.saturating_add(1);
            if self.thermal_index > 43 {
                *oxygen_stress += 2;
                self.cytochrome_c_leak = true;
                self.oxidative_efficiency *= 0.98;
            } else {
                self.cytochrome_c_leak = false;
            }
        } else {
            self.membrane_potential += 5.0;
            self.thermal_index = self.thermal_index.saturating_sub(1);
        }
    }
}

pub struct RibosomeFactory {
    pub assembly_speed: u8,
    pub active_chains: u32,
    pub error_rate: f32,
}

impl RibosomeFactory {
    pub fn setup() -> Self {
        Self {
            assembly_speed: 12,
            active_chains: 10,
            error_rate: 0.01,
        }
    }

    pub fn synthesize_proteins(&mut self, amino_pool: &mut u32, atp_pool: &mut u32, protein_storage: &mut u32) {
        if *amino_pool >= 8 && *atp_pool >= 12 {
            *amino_pool -= 8;
            *atp_pool -= 12;
            let manufactured = (self.assembly_speed as u32) * 2;
            *protein_storage += manufactured;
            self.active_chains = self.active_chains.wrapping_add(1);
        } else {
            self.active_chains = self.active_chains.saturating_sub(1);
        }
    }
}

pub struct EndoplasmicReticulum {
    pub rough_er_capacity: u32,
    pub smooth_er_lipid_flux: u32,
    pub folding_accuracy: f32,
}

impl EndoplasmicReticulum {
    pub fn build() -> Self {
        Self {
            rough_er_capacity: 500,
            smooth_er_lipid_flux: 200,
            folding_accuracy: 0.97,
        }
    }

    pub fn process_folding(&mut self, protein_storage: &mut u32, waste_pool: &mut u32) {
        if *protein_storage > 50 {
            let misfolded = (*protein_storage as f32 * (1.0 - self.folding_accuracy)) as u32;
            *protein_storage -= misfolded;
            *waste_pool += misfolded;
        }
    }
}

pub struct GolgiApparatus {
    pub vesicle_count: u32,
    pub modification_stage: u8,
    pub secretion_rate: u32,
}

impl GolgiApparatus {
    pub fn initialize() -> Self {
        Self {
            vesicle_count: 25,
            modification_stage: 3,
            secretion_rate: 10,
        }
    }

    pub fn package_proteins(&mut self, protein_storage: &mut u32, energy_pool: &mut u32) {
        if *protein_storage >= 20 && *energy_pool >= 10 {
            *protein_storage -= 20;
            *energy_pool -= 10;
            self.vesicle_count += 5;
            self.secretion_rate = self.secretion_rate.wrapping_add(2);
        } else {
            self.vesicle_count = self.vesicle_count.saturating_sub(1);
        }
    }
}

pub struct LysosomeVesicle {
    pub hydrolytic_enzyme_potency: u32,
    pub internal_acidity_ph: f32,
    pub autophagosome_count: u32,
}

impl LysosomeVesicle {
    pub fn build() -> Self {
        Self {
            hydrolytic_enzyme_potency: 85,
            internal_acidity_ph: 4.8,
            autophagosome_count: 2,
        }
    }

    pub fn digest_waste(&mut self, waste_pool: &mut u32, amino_pool: &mut u32) {
        if *waste_pool > 15 {
            *waste_pool -= 15;
            *amino_pool += 5;
            self.autophagosome_count += 1;
        } else {
            self.autophagosome_count = self.autophagosome_count.saturating_sub(1);
        }
    }
}

pub struct CytoskeletonMatrix {
    pub actin_filament_tension: f32,
    pub microtubule_integrity: f32,
    pub mechanical_stress_level: u32,
}

impl CytoskeletonMatrix {
    pub fn create() -> Self {
        Self {
            actin_filament_tension: 88.5,
            microtubule_integrity: 95.0,
            mechanical_stress_level: 10,
        }
    }

    pub fn maintain_structure(&mut self, atp_pool: &mut u32) {
        if *atp_pool >= 5 {
            *atp_pool -= 5;
            self.actin_filament_tension = 90.0;
            self.microtubule_integrity = 98.0;
        } else {
            self.actin_filament_tension *= 0.95;
            self.microtubule_integrity *= 0.96;
            self.mechanical_stress_level += 5;
        }
    }
}

pub struct PlasmaMembrane {
    pub sodium_channel_state: bool,
    pub potassium_channel_state: bool,
    pub calcium_pump_activity: u32,
    pub membrane_voltage_mv: i32,
}

impl PlasmaMembrane {
    pub fn setup() -> Self {
        Self {
            sodium_channel_state: false,
            potassium_channel_state: true,
            calcium_pump_activity: 12,
            membrane_voltage_mv: -70,
        }
    }

    pub fn handle_action_potential(&mut self, input_signal: i32) {
        self.membrane_voltage_mv += input_signal;
        if self.membrane_voltage_mv > -55 {
            self.sodium_channel_state = true;
            self.potassium_channel_state = false;
            self.membrane_voltage_mv = 40;
        } else {
            self.sodium_channel_state = false;
            self.potassium_channel_state = true;
            self.membrane_voltage_mv = -70;
        }
    }
}

pub struct CytoplasmFluidPool {
    pub glucose_moles: u32,
    pub atp_moles: u32,
    pub adp_moles: u32,
    pub amino_acid_moles: u32,
    pub protein_pool: u32,
    pub waste_moles: u32,
    pub oxygen_stress_counter: u32,
}

impl CytoplasmFluidPool {
    pub fn initialize() -> Self {
        Self {
            glucose_moles: 2000,
            atp_moles: 1000,
            adp_moles: 200,
            amino_acid_moles: 1500,
            protein_pool: 300,
            waste_moles: 10,
            oxygen_stress_counter: 0,
        }
    }

    pub fn regulate_homeostasis(&mut self) {
        if self.atp_moles < 200 {
            self.glucose_moles = self.glucose_moles.saturating_add(500);
            self.atp_moles = self.atp_moles.saturating_add(300);
        }
        if self.waste_moles > 300 {
            self.waste_moles = self.waste_moles.saturating_sub(150);
        }
    }
}

pub struct ComplexBiologicalCell {
    pub cell_id: u32,
    pub lifecycle: CellLifecycleState,
    pub cell_health: f32,
    pub cytoplasm: CytoplasmFluidPool,
    pub nucleus: CellNucleus,
    pub mitochondria_cluster: [MitochondrionUnit; 4],
    pub ribosomes: RibosomeFactory,
    pub er: EndoplasmicReticulum,
    pub golgi: GolgiApparatus,
    pub lysosome: LysosomeVesicle,
    pub cytoskeleton: CytoskeletonMatrix,
    pub membrane: PlasmaMembrane,
    pub intercellular_bus: [u32; 8],
}

impl ComplexBiologicalCell {
    pub fn boot_cell(id: u32, seed: u32) -> Self {
        Self {
            cell_id: id,
            lifecycle: CellLifecycleState::MetabolicallyActive,
            cell_health: 100.0,
            cytoplasm: CytoplasmFluidPool::initialize(),
            nucleus: CellNucleus::build(seed),
            mitochondria_cluster: [
                MitochondrionUnit::new(1),
                MitochondrionUnit::new(2),
                MitochondrionUnit::new(3),
                MitochondrionUnit::new(4),
            ],
            ribosomes: RibosomeFactory::setup(),
            er: EndoplasmicReticulum::build(),
            golgi: GolgiApparatus::initialize(),
            lysosome: LysosomeVesicle::build(),
            cytoskeleton: CytoskeletonMatrix::create(),
            membrane: PlasmaMembrane::setup(),
            intercellular_bus: [0; 8],
        }
    }

    pub fn execute_cellular_cycle(&mut self, environmental_signal: i32) {
        if self.cytoplasm.atp_moles < 30 || self.cell_health < 15.0 {
            self.lifecycle = CellLifecycleState::Apoptotic;
        }

        match self.lifecycle {
            CellLifecycleState::MetabolicallyActive => {
                for mito in self.mitochondria_cluster.iter_mut() {
                    mito.perform_oxidative_phosphorylation(
                        &mut self.cytoplasm.glucose_moles,
                        &mut self.cytoplasm.atp_moles,
                        &mut self.cytoplasm.oxygen_stress_counter,
                    );
                }

                self.ribosomes.synthesize_proteins(
                    &mut self.cytoplasm.amino_acid_moles,
                    &mut self.cytoplasm.atp_moles,
                    &mut self.cytoplasm.protein_pool,
                );

                self.nucleus.execute_transcription(
                    &mut self.cytoplasm.atp_moles,
                    &mut self.cytoplasm.amino_acid_moles,
                );

                self.er.process_folding(
                    &mut self.cytoplasm.protein_pool,
                    &mut self.cytoplasm.waste_moles,
                );

                self.golgi.package_proteins(
                    &mut self.cytoplasm.protein_pool,
                    &mut self.cytoplasm.atp_moles,
                );

                self.lysosome.digest_waste(
                    &mut self.cytoplasm.waste_moles,
                    &mut self.cytoplasm.amino_acid_moles,
                );

                self.cytoskeleton.maintain_structure(&mut self.cytoplasm.atp_moles);
                self.membrane.handle_action_potential(environmental_signal);
                self.cytoplasm.regulate_homeostasis();

                self.cytoplasm.atp_moles = self.cytoplasm.atp_moles.saturating_sub(12);
                self.cytoplasm.waste_moles += 3;

                if self.cytoplasm.oxygen_stress_counter > 15 {
                    self.lifecycle = CellLifecycleState::OxidativelyStressed;
                }
            },
            CellLifecycleState::Dormant => {
                self.cytoplasm.atp_moles = self.cytoplasm.atp_moles.saturating_sub(2);
                if self.cytoplasm.glucose_moles > 1000 {
                    self.lifecycle = CellLifecycleState::MetabolicallyActive;
                }
            },
            CellLifecycleState::OxidativelyStressed => {
                self.cell_health = (self.cell_health - 1.5).max(0.0);
                for mito in self.mitochondria_cluster.iter_mut() {
                    mito.oxidative_efficiency *= 0.97;
                }
                if self.cytoplasm.atp_moles > 600 {
                    self.lifecycle = CellLifecycleState::MetabolicallyActive;
                    self.cytoplasm.oxygen_stress_counter = 0;
                }
            },
            CellLifecycleState::Proliferating => {
                if self.cytoplasm.atp_moles > 1200 && self.cytoplasm.amino_acid_moles > 800 {
                    self.cytoplasm.atp_moles -= 500;
                    self.cytoplasm.amino_acid_moles -= 400;
                    self.nucleus.dna.mutate_sequence(self.cell_id as u8);
                    self.lifecycle = CellLifecycleState::MetabolicallyActive;
                } else {
                    self.lifecycle = CellLifecycleState::MetabolicallyActive;
                }
            },
            CellLifecycleState::Senescent => {
                self.cell_health = (self.cell_health - 0.2).max(0.0);
            },
            CellLifecycleState::Apoptotic => {
                self.cytoplasm.atp_moles = 0;
                self.cell_health = 0.0;
            },
        }
    }

    pub fn receive_network_broadcast(&mut self, incoming_bus: &[u32; 8]) {
        for i in 0..8 {
            self.intercellular_bus[i] = incoming_bus[i] ^ (self.cell_id.wrapping_mul(31));
            if self.intercellular_bus[i] > 2000 {
                self.lifecycle = CellLifecycleState::OxidativelyStressed;
            }
        }
    }

    pub fn trigger_cellular_regeneration(&mut self) {
        if self.lifecycle == CellLifecycleState::Apoptotic {
            self.cytoplasm = CytoplasmFluidPool::initialize();
            self.cell_health = 100.0;
            self.lifecycle = CellLifecycleState::MetabolicallyActive;
        }
    }
}

pub struct BiologicalTissueNetwork {
    pub cellular_grid: [ComplexBiologicalCell; 16],
    pub tissue_signal_bus: [u32; 8],
    pub temporal_tick_counter: u64,
}

impl BiologicalTissueNetwork {
    pub fn construct_network() -> Self {
        let mut grid = [
            ComplexBiologicalCell::boot_cell(1, 101),
            ComplexBiologicalCell::boot_cell(2, 202),
            ComplexBiologicalCell::boot_cell(3, 303),
            ComplexBiologicalCell::boot_cell(4, 404),
            ComplexBiologicalCell::boot_cell(5, 505),
            ComplexBiologicalCell::boot_cell(6, 606),
            ComplexBiologicalCell::boot_cell(7, 707),
            ComplexBiologicalCell::boot_cell(8, 808),
            ComplexBiologicalCell::boot_cell(9, 909),
            ComplexBiologicalCell::boot_cell(10, 1010),
            ComplexBiologicalCell::boot_cell(11, 1111),
            ComplexBiologicalCell::boot_cell(12, 1212),
            ComplexBiologicalCell::boot_cell(13, 1313),
            ComplexBiologicalCell::boot_cell(14, 1414),
            ComplexBiologicalCell::boot_cell(15, 1515),
            ComplexBiologicalCell::boot_cell(16, 1616),
        ];

        for i in 0..16 {
            grid[i] = ComplexBiologicalCell::boot_cell(i as u32 + 1, (i as u32 + 1) * 77);
        }

        Self {
            cellular_grid: grid,
            tissue_signal_bus: [123, 456, 789, 321, 654, 987, 111, 222],
            temporal_tick_counter: 0,
        }
    }

    pub fn advance_tissue_simulation(&mut self) {
        self.temporal_tick_counter = self.temporal_tick_counter.wrapping_add(1);

        for i in 0..16 {
            let environmental_impulse = ((self.temporal_tick_counter + i as u64) % 12) as i32 - 6;
            self.cellular_grid[i].execute_cellular_cycle(environmental_impulse);
            self.cellular_grid[i].trigger_cellular_regeneration();
        }

        for i in 0..16 {
            let aggregate_output = self.cellular_grid[i].cytoplasm.atp_moles ^ self.cellular_grid[i].cell_id;
            let bus_index = i % 8;
            self.tissue_signal_bus[bus_index] = aggregate_output;

            let target_neighbor = (i + 1) % 16;
            self.cellular_grid[target_neighbor].receive_network_broadcast(&self.tissue_signal_bus);
        }
    }
}

fn main() {
    let mut engineered_tissue = BiologicalTissueNetwork::construct_network();

    for _ in 0..100 {
        engineered_tissue.advance_tissue_simulation();
        thread::sleep(Duration::from_millis(10));
    }
}
