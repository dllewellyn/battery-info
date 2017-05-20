
#[link(name = "battery_info_lib")]
extern {
    fn batteryLevel() -> f64;
}

pub fn get_battery() -> f64 {

    #[allow(unused_assignments)]
    let mut result : f64 = 0.0;

    unsafe {
        result = batteryLevel();
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
                println!("{}", &cap[1]);

                let result = cap[1].to_string().parse::<f64>().unwrap();
                assert_eq!(result, battery_result);
            }
        } else {
            panic!("Unsupported platform");
        }
    }
}

