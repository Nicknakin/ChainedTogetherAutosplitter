use super::settings::Settings;
use super::state::State;

use super::Square;

trait Split {
    fn should_split(&self, state: &State) -> bool;
}

pub struct SphereTrigger {
    position_x: f64,
    position_y: f64,
    position_z: f64,
    radius_squared: f64,
}

impl SphereTrigger {
    pub fn new(
        position_x: f64,
        position_y: f64,
        position_z: f64,
        radius_squared: f64,
    ) -> SphereTrigger {
        return SphereTrigger {
            position_x,
            position_y,
            position_z,
            radius_squared,
        };
    }
    pub fn should_split(&self, state: &State) -> bool {
        let dist_squared = (self.position_x - state.position_x).square()
            + (self.position_y - state.position_y).square()
            + (self.position_z - state.position_z).square();
        return dist_squared <= self.radius_squared;
    }
}

pub struct UpperSphereTrigger {
    position_x: f64,
    position_y: f64,
    position_z: f64,
    radius_squared: f64,
}

impl UpperSphereTrigger {
    pub fn new(
        position_x: f64,
        position_y: f64,
        position_z: f64,
        radius_squared: f64,
    ) -> UpperSphereTrigger {
        return UpperSphereTrigger {
            position_x,
            position_y,
            position_z,
            radius_squared,
        };
    }
    pub fn should_split(&self, state: &State) -> bool {
        let dist_squared = (self.position_x - state.position_x).square()
            + (self.position_y - state.position_y).square()
            + (self.position_z - state.position_z).square();
        return dist_squared <= self.radius_squared && self.position_z <= state.position_z;
    }
}

pub struct BoxTrigger {
    position_x_1: f64,
    position_y_1: f64,
    position_x_2: f64,
    position_y_2: f64,
}

impl BoxTrigger {
    pub fn new(
        position_x_1: f64,
        position_x_2: f64,
        position_y_1: f64,
        position_y_2: f64,
    ) -> BoxTrigger {
        return BoxTrigger {
            position_x_1,
            position_x_2,
            position_y_1,
            position_y_2,
        };
    }

    pub fn should_split(&self, state: &State) -> bool {
        return state.position_x >= self.position_x_1
            && state.position_x <= self.position_x_2
            && state.position_y >= self.position_y_1
            && state.position_y <= self.position_y_2;
    }
}

pub struct HeightTrigger {
    position_z: f64,
}

impl HeightTrigger {
    pub fn new(position_z: f64) -> HeightTrigger {
        return HeightTrigger { position_z };
    }
    pub fn should_split(&self, state: &State) -> bool {
        return state.position_z >= self.position_z;
    }
}

#[derive(Clone)]
pub enum Triggers {
    Underworld,
    FirstLadder,
    HellCliffs,
    HellCliffsDrone,
    TheCarRace,
    RotatingCube,
    TheWhisperingVault,
    OpenTheDoor,
    TheAquaMaze,
    TheSubwayStation,
    StationDrone,
    RedElevatortoCity,
    TheCity,
    CityCrane,
    DoubleCranePlatforms,
    ElevatortoBuildings,
    OverTheBuildings,
    BuildingsHelicopter,
    TheWarehouse,
    TheHarbor,
    BlueRailingChariot,
    Elevatortostairs,
    HelicoptertoTemple,
    TheTemple,
    ElevatortoAsianShrine,
    TheAsianShrine,
    WoodenHorseWagon,
    TheDeities,
    WoodenBoat,
    ZeusLightning,
    AirBalloonPump,
    TheGarden,
    TheFinalTrials,
    Kiosk,
    Carriage,
    TheSun,
}

impl Triggers {
    pub fn should_split(&self, state: &State) -> bool {
        let trigger = match *self {
            Triggers::Underworld => {
                SphereTrigger::new(48169.70f64, -6670.38f64, 10415.32f64, 1600f64 * 1600f64)
                    .should_split(state)
            }
            Triggers::FirstLadder => {
                BoxTrigger::new(57607f64, 57647f64, -4686f64, -4602f64).should_split(state)
            }
            Triggers::HellCliffs => {
                SphereTrigger::new(58518.04f64, -5896.68f64, 22762.73f64, 600f64 * 600f64)
                    .should_split(state)
            }
            Triggers::HellCliffsDrone => {
                SphereTrigger::new(63361.97f64, -7714.82f64, 28700f64, 150f64 * 150f64)
                    .should_split(state)
            }
            Triggers::TheCarRace => {
                SphereTrigger::new(60736f64, -5806f64, 34473f64, 2000f64 * 2000f64)
                    .should_split(state)
            }
            Triggers::RotatingCube => {
                SphereTrigger::new(57840.65f64, -4092.85f64, 40061.93f64, 550f64 * 550f64)
                    .should_split(state)
            }
            Triggers::TheWhisperingVault => HeightTrigger::new(47380f64).should_split(state),
            Triggers::OpenTheDoor => {
                SphereTrigger::new(70099.82f64, -12091.44f64, 54309.28f64, 1000f64 * 1000f64)
                    .should_split(state)
            }
            Triggers::TheAquaMaze => {
                SphereTrigger::new(56175f64, -9616f64, 61161f64, 1900f64 * 1900f64)
                    .should_split(state)
            }
            Triggers::TheSubwayStation => {
                SphereTrigger::new(61164.22f64, -16430.15f64, 66112.40f64, 875f64 * 875f64)
                    .should_split(state)
            }
            Triggers::StationDrone => {
                SphereTrigger::new(43285.67f64, -19387.02f64, 71659.89f64, 85f64 * 85f64)
                    .should_split(state)
            }
            Triggers::RedElevatortoCity => {
                SphereTrigger::new(55934.05f64, -7474.58f64, 85888.98f64, 360f64 * 360f64)
                    .should_split(state)
            }
            Triggers::TheCity => HeightTrigger::new(114310f64).should_split(state),
            Triggers::CityCrane => {
                SphereTrigger::new(64600f64, -5950f64, 119450f64, 2340f64 * 2340f64)
                    .should_split(state)
            }
            Triggers::DoubleCranePlatforms => {
                SphereTrigger::new(54500f64, -9000f64, 125818f64, 2100f64 * 2100f64)
                    .should_split(state)
            }
            Triggers::ElevatortoBuildings => {
                SphereTrigger::new(59656f64, -7845f64, 133400f64, 1050f64 * 1050f64)
                    .should_split(state)
            }
            Triggers::OverTheBuildings => HeightTrigger::new(139850f64).should_split(state),
            Triggers::BuildingsHelicopter => {
                SphereTrigger::new(61241f64, -7060f64, 149050f64, 150f64 * 150f64)
                    .should_split(state)
            }
            Triggers::TheWarehouse => HeightTrigger::new(155996f64).should_split(state),
            Triggers::TheHarbor => {
                UpperSphereTrigger::new(58833.19f64, -10168.21f64, 167476.00f64, 4100f64 * 4100f64)
                    .should_split(state)
            }
            Triggers::BlueRailingChariot => {
                SphereTrigger::new(55267f64, -9424f64, 175450f64, 2200f64 * 2200f64)
                    .should_split(state)
            }
            Triggers::Elevatortostairs => {
                SphereTrigger::new(60015f64, -5540f64, 180200f64, 1550f64 * 1550f64)
                    .should_split(state)
            }
            Triggers::HelicoptertoTemple => {
                SphereTrigger::new(63676.359f64, -12937.55f64, 203690f64, 150f64 * 150f64)
                    .should_split(state)
            }
            Triggers::TheTemple => HeightTrigger::new(209230f64).should_split(state),
            Triggers::ElevatortoAsianShrine => {
                SphereTrigger::new(60844f64, -13114f64, 233000f64, 1800f64 * 1800f64)
                    .should_split(state)
            }
            Triggers::TheAsianShrine => HeightTrigger::new(244140f64).should_split(state),
            Triggers::WoodenHorseWagon => {
                SphereTrigger::new(57636f64, -9231f64, 270700f64, 1300f64 * 1300f64)
                    .should_split(state)
            }
            Triggers::TheDeities => {
                SphereTrigger::new(40703.73f64, -1902.68f64, 278480.62f64, 2270f64 * 2270f64)
                    .should_split(state)
            }
            Triggers::WoodenBoat => {
                SphereTrigger::new(52432f64, -1512f64, 284710f64, 1600f64 * 1600f64)
                    .should_split(state)
            }
            Triggers::ZeusLightning => {
                SphereTrigger::new(81930f64, -23160f64, 301710f64, 610f64 * 610f64)
                    .should_split(state)
            }
            Triggers::AirBalloonPump => {
                SphereTrigger::new(45515f64, -11171f64, 307250f64, 100f64 * 100f64)
                    .should_split(state)
            }
            Triggers::TheGarden => {
                SphereTrigger::new(47610.46f64, -11083.54f64, 334514.26f64, 4200f64 * 4200f64)
                    .should_split(state)
            }
            Triggers::TheFinalTrials => {
                SphereTrigger::new(56907f64, -32509f64, 337997f64, 940f64 * 940f64)
                    .should_split(state)
            }
            Triggers::Kiosk => SphereTrigger::new(56351f64, -23282f64, 351400f64, 650f64 * 650f64)
                .should_split(state),
            Triggers::Carriage => {
                SphereTrigger::new(60435f64, -27560f64, 357470f64, 450f64 * 450f64)
                    .should_split(state)
            }
            Triggers::TheSun => {
                SphereTrigger::new(89000f64, -4500f64, 373000f64, 8500f64 * 8500f64)
                    .should_split(state)
            }
        };
        return trigger;
    }
    pub fn next_trigger_raw(&self) -> Triggers {
        match self {
            Triggers::Underworld => Triggers::FirstLadder,
            Triggers::FirstLadder => Triggers::HellCliffs,
            Triggers::HellCliffs => Triggers::HellCliffsDrone,
            Triggers::HellCliffsDrone => Triggers::TheCarRace,
            Triggers::TheCarRace => Triggers::RotatingCube,
            Triggers::RotatingCube => Triggers::TheWhisperingVault,
            Triggers::TheWhisperingVault => Triggers::OpenTheDoor,
            Triggers::OpenTheDoor => Triggers::TheAquaMaze,
            Triggers::TheAquaMaze => Triggers::TheSubwayStation,
            Triggers::TheSubwayStation => Triggers::StationDrone,
            Triggers::StationDrone => Triggers::RedElevatortoCity,
            Triggers::RedElevatortoCity => Triggers::TheCity,
            Triggers::TheCity => Triggers::CityCrane,
            Triggers::CityCrane => Triggers::DoubleCranePlatforms,
            Triggers::DoubleCranePlatforms => Triggers::ElevatortoBuildings,
            Triggers::ElevatortoBuildings => Triggers::OverTheBuildings,
            Triggers::OverTheBuildings => Triggers::BuildingsHelicopter,
            Triggers::BuildingsHelicopter => Triggers::TheWarehouse,
            Triggers::TheWarehouse => Triggers::TheHarbor,
            Triggers::TheHarbor => Triggers::BlueRailingChariot,
            Triggers::BlueRailingChariot => Triggers::Elevatortostairs,
            Triggers::Elevatortostairs => Triggers::HelicoptertoTemple,
            Triggers::HelicoptertoTemple => Triggers::TheTemple,
            Triggers::TheTemple => Triggers::ElevatortoAsianShrine,
            Triggers::ElevatortoAsianShrine => Triggers::TheAsianShrine,
            Triggers::TheAsianShrine => Triggers::WoodenHorseWagon,
            Triggers::WoodenHorseWagon => Triggers::TheDeities,
            Triggers::TheDeities => Triggers::WoodenBoat,
            Triggers::WoodenBoat => Triggers::ZeusLightning,
            Triggers::ZeusLightning => Triggers::AirBalloonPump,
            Triggers::AirBalloonPump => Triggers::TheGarden,
            Triggers::TheGarden => Triggers::TheFinalTrials,
            Triggers::TheFinalTrials => Triggers::Kiosk,
            Triggers::Kiosk => Triggers::Carriage,
            Triggers::Carriage => Triggers::TheSun,
            Triggers::TheSun => Triggers::TheSun,
        }
    }
    pub fn next_trigger(&self, settings: &Settings) -> Triggers {
        let mut trigger = self.next_trigger_raw();
        while !trigger.is_enabled(settings) {
            trigger = trigger.next_trigger_raw();
        }
        trigger
    }
    pub fn first_trigger(settings: &Settings) -> Triggers {
        let mut trigger = Triggers::Underworld;
        while !trigger.is_enabled(settings) {
            trigger = trigger.next_trigger_raw();
        }
        trigger
    }
    pub fn is_enabled(&self, settings: &Settings) -> bool {
        match self {
            Triggers::Underworld => settings.toggle_underworld,
            Triggers::FirstLadder => settings.toggle_first_ladder,
            Triggers::HellCliffs => settings.toggle_hell_cliffs,
            Triggers::HellCliffsDrone => settings.toggle_hell_cliffs_drone,
            Triggers::TheCarRace => settings.toggle_the_car_race,
            Triggers::RotatingCube => settings.toggle_rotating_cube,
            Triggers::TheWhisperingVault => settings.toggle_the_whispering_vault,
            Triggers::OpenTheDoor => settings.toggle_open_the_door,
            Triggers::TheAquaMaze => settings.toggle_the_aqua_maze,
            Triggers::TheSubwayStation => settings.toggle_the_subway_station,
            Triggers::StationDrone => settings.toggle_station_drone,
            Triggers::RedElevatortoCity => settings.toggle_red_elevator_to_city,
            Triggers::TheCity => settings.toggle_the_city,
            Triggers::CityCrane => settings.toggle_city_crane,
            Triggers::DoubleCranePlatforms => settings.toggle_double_crane_platforms,
            Triggers::ElevatortoBuildings => settings.toggle_elevator_to_buildings,
            Triggers::OverTheBuildings => settings.toggle_over_the_buildings,
            Triggers::BuildingsHelicopter => settings.toggle_buildings_helicopter,
            Triggers::TheWarehouse => settings.toggle_the_warehouse,
            Triggers::TheHarbor => settings.toggle_the_harbor,
            Triggers::BlueRailingChariot => settings.toggle_blue_railing_chariot,
            Triggers::Elevatortostairs => settings.toggle_elevator_to_stairs,
            Triggers::HelicoptertoTemple => settings.toggle_helicopter_to_temple,
            Triggers::TheTemple => settings.toggle_the_temple,
            Triggers::ElevatortoAsianShrine => settings.toggle_elevator_to_asian_shrine,
            Triggers::TheAsianShrine => settings.toggle_the_asian_shrine,
            Triggers::WoodenHorseWagon => settings.toggle_wooden_horse_wagon,
            Triggers::TheDeities => settings.toggle_the_deities,
            Triggers::WoodenBoat => settings.toggle_wooden_boat,
            Triggers::ZeusLightning => settings.toggle_zeus_lightning,
            Triggers::AirBalloonPump => settings.toggle_air_balloon_pump,
            Triggers::TheGarden => settings.toggle_the_garden,
            Triggers::TheFinalTrials => settings.toggle_the_final_trials,
            Triggers::Kiosk => settings.toggle_kiosk,
            Triggers::Carriage => settings.toggle_carriage,
            Triggers::TheSun => settings.toggle_the_sun,
        }
    }
}
