use std::fs::read;
use psfparser::binary::parse;
use psfparser::analysis::transient::TransientData;
use rust_xlsxwriter::worksheet::Worksheet;
use rust_xlsxwriter::workbook::Workbook;
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::collections::BTreeMap;

// The parse fn takes the filepath of transient data and outputs a vector of delay-dout pairs.
fn parse_vec(path: String, delay: f64) -> Vec<(f64, u32)>{
    let mut parsed_vec: Vec<(f64, u32)> = Vec::new();
    let mut sum: u32 = 0;
    let read_bytes = read(path).unwrap();
    let parse_bytes = parse(&read_bytes).unwrap();
    let trans_data = TransientData::from_binary(parse_bytes);
    for i in 0..252 {
        let name = format!("dout{i}");
        let dout_i: &Vec<f64> = &trans_data.signals[&name];
        let probe = *(dout_i.last().unwrap());
        let dout: u32 = if probe > 1.7 {
            1
        } else if probe < 0.1 {
            0
        } else {
            panic!("dout was not close to either 0 or 1");
        };
        sum += dout;
    }
    parsed_vec.push((delay, sum));
    parsed_vec
}

// The aggregate fn takes a parsed vector input and maps vector's delay (val) into one unique code (dout/key) accordingly.
// Example: [(0.1, 1)] and [(0.2, 1)] -> [1, (0.1, 0.2)]
fn aggregate(input: &Vec<(f64, u32)>, full_data: &mut BTreeMap<u32, Vec<f64>>) {
	for (delay, code) in input.iter() {
		full_data.entry(*code).or_insert_with(Vec::new).push(*delay);
	}
}

// The Range struct contains starting and ending point of delay corresponding to each dout.
// Example: (1, [0.1, 0.15, 0.2]) -> dout = 1, start = 0.1, end = 0.2
#[derive(Debug)]
#[derive(Clone)]
struct Range {
	start: f64,
	end: f64
}

// The extract fn takes the dataset and extract the first and last occurrence of delay for each unique code
fn extract(full_data: BTreeMap<u32, Vec<f64>>, extracted_data: &mut BTreeMap<u32, Range>) {
	for (code, delay) in full_data.iter() {
        let min_delay = delay.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).map(|&x| x).unwrap_or(0.0);
        let max_delay = delay.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).map(|&x| x).unwrap_or(0.0);
		extracted_data.insert(*code, Range {
			start: min_delay,
			end: max_delay
		});
	}
}

// The pedestal_width fn takes the extracted dataset and compute pedestal width for each set
fn pedestal_width(input: BTreeMap<u32, Range>) -> Vec<(u32, f64)> {
    let mut widths = Vec::new();
	for code in 1..=252 {
		let last = input[&(code-1)].end;
		let first = input[&code].start;
		let width: f64 = (((first + last) / 2.0) * 10f64.powi(4)).round() / 10f64.powi(4);
        widths.push((code, width));
	}
    widths
}

fn main() {
    let mut lin_sweep = 2500;
    let mut full_data: BTreeMap<u32, Vec<f64>> = BTreeMap::new();
    let mut extracted_data = BTreeMap::new();
    for sweep_count in 0..=lin_sweep {
        // STEP 1: Parse number of lin_sweep
        let sweep_count_format = format!("{:03}", sweep_count);
        let delay = ((sweep_count as f64 * (7.0 / lin_sweep as f64)) * 10f64.powi(4)).round() / 10f64.powi(4);
        let path = format!("/tools/scratch/dwzhang/tdc_sky130_macros/tdc_64/tdc_64_sim.raw/sweepDelay-{sweep_count_format}_stepResponse.tran.tran");
        let parsed_vec: Vec<(f64, u32)> = parse_vec(path, delay);
        
        // STEP 2: Aggregate available sweeped data
        aggregate(&parsed_vec, &mut full_data);
        // println!("{:?}", full_data);

        // STEP 3: Extract data to get first and last occurrence of each set
        extract(full_data.clone(), &mut extracted_data);
        // println!("{:?}", extracted_data);
    }
    // STEP 4: Generate pedestal width between each set
    let result = pedestal_width(extracted_data.clone());
    println!("{:?}", result);

}
