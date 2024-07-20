use super::trigger::Triggers;
use super::Square;
use asr::Process;

#[derive(Default, Clone)]
pub struct State {
    pub position_x: f64,
    pub position_y: f64,
    pub position_z: f64,

    pub timer: u32,
}

impl State {
    pub fn generate(process: &Process) -> Result<Option<State>, asr::Error> {
        const POSITION_X_PATH: &[u64] = &[0x06F67C48, 0x1B8, 0x38, 0x0, 0x30, 0x2D8, 0x1A0, 0x128];
        const POSITION_Y_PATH: &[u64] = &[0x06F67C48, 0x1B8, 0x38, 0x0, 0x30, 0x2D8, 0x1A0, 0x130];
        const POSITION_Z_PATH: &[u64] = &[0x06F67C48, 0x1B8, 0x38, 0x0, 0x30, 0x2D8, 0x1A0, 0x138];
        const TIMER_PATH: &[u64] = &[0x06F67C48, 0x180, 0x8, 0x320];

        asr::print_message("Extracting x pos");
        let position_x = process.read_pointer_path(
            process.get_module_address("ChainedTogether-Win64-Shipping.exe")?,
            asr::PointerSize::Bit64,
            POSITION_X_PATH,
        )?;
        asr::print_message("Extracting y pos");
        let position_y = process.read_pointer_path(
            process.get_module_address("ChainedTogether-Win64-Shipping.exe")?,
            asr::PointerSize::Bit64,
            POSITION_Y_PATH,
        )?;
        asr::print_message("Extracting z pos");
        let position_z = process.read_pointer_path(
            process.get_module_address("ChainedTogether-Win64-Shipping.exe")?,
            asr::PointerSize::Bit64,
            POSITION_Z_PATH,
        )?;
        asr::print_message("Extracting timer");
        let timer = process.read_pointer_path(
            process.get_module_address("ChainedTogether-Win64-Shipping.exe")?,
            asr::PointerSize::Bit64,
            TIMER_PATH,
        )?;

        Ok(Some(State {
            position_x,
            position_y,
            position_z,
            timer,
        }))
    }

    pub fn log(&self) {
        asr::print_limited::<1024>(&format_args!(
            "({:.2?}, {:.2?}, {:.2?}) ({:?}:{:02x})",
            self.position_x,
            self.position_y,
            self.position_z,
            self.timer / 60,
            self.timer % 60
        ));
    }

    pub fn should_start(&self, old_state: &State) -> bool {
        old_state.timer == 0 && self.timer != 0
    }

    pub fn should_split(&mut self, current_trigger: &Triggers) -> bool {
        current_trigger.should_split(self)
    }

    pub fn should_reset(&self) -> bool {
        self.timer == 0
            && ((((self.position_x - 66649.54).square()
                + (self.position_y + 7418.37).square()
                + (self.position_z - 3118.52).square())
                <= 1f64)
                || (self.position_x == 0f64 && self.position_y == 0f64 && self.position_z == 0f64))
    }
}
