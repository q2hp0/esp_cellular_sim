// controls cross-membrane ion gradients and electrochemical action potentials
pub struct ion_channel {
    pub voltage_gate: i32,
    pub open_state: bool,
}

impl ion_channel {
    pub fn factory_reset() -> self {
        self {
            voltage_gate: -70,
            open_state: false,
        }
    }

    pub fn trigger_action_potential(&mut self, signal_in: i32) {
        self.voltage_gate += signal_in;
        if self.voltage_gate > -50 {
            self.open_state = true;
            self.voltage_gate = 40;
        } else {
            self.open_state = false;
            self.voltage_gate = -70;
        }
    }
}
