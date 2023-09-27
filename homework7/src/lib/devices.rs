use crate::power_plug::PowerPlug;
use crate::{Info, Report};

impl Report for PowerPlug {
    fn string_report(&self) -> String {
        let mut plug_report = String::from("Plug name: ");
        plug_report.push_str(self.get_name().as_str());
        plug_report
            .push_str(format!("Current consumption: {}", self.current_consumption()).as_str());
        plug_report.push_str(format!("Current state: {}", self.get_state()).as_str());
        plug_report
    }
}

