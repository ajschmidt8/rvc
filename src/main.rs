use std::env;
/*
0.16
0.17
0.18
0.19
21.6  | 0.20 | 0
21.8  | 0.21 | 1
21.10 | 0.22 | 2
21.12 | 0.23 | 3
22.02 | 0.24 | 4
*/

// Print usage info if no arguments are provided.

fn main() -> Result<(), &'static str> {
    let args = env::args();
    let args_count = args.len();
    if args_count != 2 {
        return Err("Please provide a single argument.");
    }
    let args_list: Vec<String> = args.collect();
    let input_version = &args_list[1];
    let version_components = input_version.split(".").collect::<Vec<&str>>();
    if version_components.len() != 2 {
        return Err("Only '<major>.<minor>' format is supported.");
    }
    let major_version = version_components[0]
        .parse::<usize>()
        .expect("Unable to parse major version.");
    let minor_version = version_components[1]
        .parse::<usize>()
        .expect("Unable to parse minor version.");

    let output_version = {
        match major_version {
            0 => ucx_to_rapids(minor_version),
            21.. => rapids_to_ucx(major_version, minor_version),
            _ => return Err("Major version should be 0 or >=21"),
        }
    };
    println!("input version: {}", input_version);
    println!("output version: {}", output_version);
    Ok(())
}

fn ucx_to_rapids(ucx_version_number: usize) -> String {
    let ucx_py_offset = 20;
    let version_difference = ucx_version_number - ucx_py_offset;
    // check that version difference is not negative
    let number_of_months = 12;
    let initial_major_version = 21;
    let initial_minor_version = 6;

    let major_version = (((version_difference as f64 * 2.0) + initial_minor_version as f64)
        / number_of_months as f64)
        .floor()
        + initial_major_version as f64;

    let mut minor_version = ((version_difference * 2) + 6) % number_of_months;

    match minor_version {
        0 => minor_version = 12,
        n => minor_version = n,
    }

    format!("{}.{:02}", major_version, minor_version)
}

fn rapids_to_ucx(major_version: usize, minor_version: usize) -> String {
    // add some checks for major/minor version limits?
    // add checks for odd numbers (i.e. 21.05)
    let ucx_py_offset = 20;
    let number_of_months = 12;
    let initial_major_version = 21;
    let initial_minor_version = 6;
    let major_difference = major_version - initial_major_version;

    let ucx_version =
        (((number_of_months * major_difference) + minor_version - initial_minor_version) / 2)
            + ucx_py_offset;
    format!("0.{}", ucx_version)
}
