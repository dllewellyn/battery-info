extern crate regex;

use std::path::Path;
use std::fs;
use std::io::Read;
use std::fs::File;
use self::regex::Regex;

#[cfg(all(target_os="macos"))]
#[link(name = "battery_info_lib")]
extern {
    fn batteryLevel() -> f64;
}

#[cfg(all(target_os="linux"))]
fn batteryLevel() -> f64 {
  panic!("Should not be calling stub function");
  return 0.0;
}

#[cfg(not(macos))]
pub fn battery_level_linux() -> f64 {
    let path = Path::new("/proc/acpi/battery/");
    if path.exists() && path.is_dir() {

        let paths = fs::read_dir("/proc/acpi/battery/").unwrap();

        for path in paths {
                let battery_path_temp = path.unwrap().path();
                let battery_path = battery_path_temp.as_path();

                if battery_path.exists() && battery_path.is_dir() {
                    let info_path = battery_path.join("info");
                    let status_path = battery_path.join("state");


                    let info_file = File::open(info_path.to_str().unwrap());
                    let status_file = File::open(status_path.to_str().unwrap());


                    if info_file.is_ok() && status_file.is_ok() {

                        // Open up the info and status for this battery
                        // Info will give us the max capacity, e.g.:
                        //
                        // Last full capacity:      5000 mAh
                        //
                        // Status will give us the remaining. E.g.:
                        //
                        // remaining capacity:      4850 mAh
                        let mut info = String::new();
                        let mut status = String::new();

                        info_file.unwrap().read_to_string(&mut info).unwrap();
                        status_file.unwrap().read_to_string(&mut status).unwrap();

                        // Regex those two files to extract the data we need.
                        let re_info = Regex::new(r"last full capacity:[ ]+([0-9]+) [A-Za-z]+").unwrap();
                        let info_results = re_info.captures(info.as_str());

                        let re_status = Regex::new(r"remaining capacity:[ ]+([0-9]+) [A-Za-z]+").unwrap();
                        let status_results = re_status.captures(status.as_str());

                        // Battery percentage is: (max / 100) * current
                        return (100.00 / info_results.unwrap().get(1).unwrap().as_str().parse::<f64>().unwrap())
                        * status_results.unwrap().get(1).unwrap().as_str().parse::<f64>().unwrap()

                    }


                }
            }


    }

    return 0.0;
}



pub fn get_battery() -> f64 {

    #[allow(unused_assignments)]
    let mut result : f64 = 0.0;

    if cfg!(target_os = "macos") {
        unsafe {
            result = batteryLevel()
        };
    } else if cfg!(target_os = "linux") {
            result = battery_level_linux()
    };

    return result;
}

#[cfg(test)]
mod tests {

    use get_battery;
    use std::process::Command;
    extern crate regex;
    use self::regex::Regex;

    #[test]
    fn check_that_callthrough_to_battery_works() {
        let battery_result = get_battery();

        if cfg!(target_os = "macos") {
            let output = Command::new("pmset")
                .args(vec!["-g", "batt"])
                .output()
                .expect("failed to execute process");

            let data = String::from_utf8(output.stdout).unwrap();
            println!("{}", data);

            let re = Regex::new(r"([0-9]{1,3})%").unwrap();
            let regex_results = re.captures_iter(data.as_str());
            let count = regex_results.count();
            assert!(count > 0);

            for cap in re.captures_iter(data.as_str()) {
                let result = cap[1].to_string().parse::<f64>().unwrap();
                assert_eq!(result, battery_result);
            }
        } else if cfg!(target_os = "linux") {
            let output = Command::new("upower")
                .args(vec!["-d", "|", "grep", "\"percentage\""])
                .output()
                .expect("failed to execute process");
            let re = Regex::new(r"([0-9]{1,3})%").unwrap();
            let data = String::from_utf8(output.stdout).unwrap();
            let captures = re.captures(data.as_str()).unwrap();
            assert_eq!(battery_result, captures.get(1).unwrap().as_str().parse::<f64>().unwrap());

        } else {
            panic!("Unsupported platform");
        }
    }
}

