use psfparser::analysis::transient::TransientData;
use psfparser::binary::parse;
use rust_decimal::Decimal;
use rust_xlsxwriter::workbook::Workbook;
use rust_xlsxwriter::worksheet::Worksheet;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs::read;

// The parse fn takes the filepath of transient data and outputs a vector of delay-dout pairs.
fn parse_vec(path: String, delay: f64) -> Vec<(f64, u32)> {
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

// The Range struct contains starting and ending point of delay corresponding to each code.
// Example: (1, [0.1, 0.15, 0.2]) -> dout = 1, start = 0.1, end = 0.2
#[derive(Debug, Clone)]
struct Range {
    start: f64,
    end: f64,
}

// The extract fn takes the dataset and extract the first and last occurrence of delay for each unique code.
fn extract(full_data: BTreeMap<u32, Vec<f64>>, extracted_data: &mut BTreeMap<u32, Range>) {
    for (code, delay) in full_data.iter() {
        let min_delay = delay
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .map(|&x| x)
            .unwrap_or(0.0);
        let max_delay = delay
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .map(|&x| x)
            .unwrap_or(0.0);
        extracted_data.insert(
            *code,
            Range {
                start: min_delay,
                end: max_delay,
            },
        );
    }
}

// The transition_pt fn takes the extracted dataset and compute an estimate transition for each code.
fn transition_pt(input: BTreeMap<u32, Range>) -> Vec<(u32, f64)> {
    let mut widths = Vec::new();
    for code in 1..=252 {
        let last = input[&(code - 1)].end;
        let first = input[&code].start;
        let width: f64 = ((first + last) / 2.0);
        widths.push((code, width));
    }
    widths
}

// The dnl_gen takes the transition point dataset and generate dnl dataset.
fn dnl_gen(input: Vec<(u32, f64)>) -> Vec<(u32, f64)> {
    // 1. Take the first and the last transition of the transtion dataset to find average code width.
    let first_trans = input.first().map(|&(_, num)| num).unwrap();
    let last_trans = input.last().map(|&(_, num)| num).unwrap();
    let avg_code_width = (last_trans - first_trans) / 251.0;    // not rounding
    println!("{:?}", avg_code_width);

    // 2. Then, compute the pedestal width for each code using the pedestal transition.
    let mut data_widths = Vec::new();
    for code in 1..=251 {
        let end = input[code - 1].1;
        let front = input[code].1;
        let width: f64 = (front - end);
        println!("{:?}", width);
        data_widths.push((code as u32, width));
    }

    // 3. Lastly, compute dnl using each code's width and the average code width.
    let mut dnl_data = Vec::new();
    for code in 1..=251 {
        let dnl = (data_widths[code - 1].1 - avg_code_width) / avg_code_width;      // not rounding
        dnl_data.push((code as u32, dnl));
    }
    dnl_data
}

// The inl_gen takes dnl dataset and generate inl dataset.
fn inl_gen(input: Vec<(u32, f64)>) -> Vec<(u32, f64)> {
    let mut inl_data = Vec::new();
    let mut inl = 0.0;
    for code in 1..=252 {
        let dnl: f64 = if code < 252 { 
            input[code - 1].1
        } else {
            0.0
        };
        inl_data.push((code as u32, inl));
        inl = dnl + inl;
    }
    inl_data
}

fn main() {
    let mut lin_sweep = 2500;
    let mut full_data: BTreeMap<u32, Vec<f64>> = BTreeMap::new();
    let mut extracted_data = BTreeMap::new();
    for sweep_count in 0..=lin_sweep {
        // STEP 1: Parse number of lin_sweep.
        let sweep_count_format = format!("{:03}", sweep_count);
        let delay = (sweep_count as f64 * (10.0 / lin_sweep as f64));
        let path = format!("/tools/scratch/dwzhang/tdc_sky130_macros/tdc_64/nda/tdc_64_pex_sim.raw/sweepDelay-{sweep_count_format}_stepResponse.tran.tran");
        let parsed_vec: Vec<(f64, u32)> = parse_vec(path, delay);
        // println!("{:?}", parsed_vec);

        // STEP 2: Aggregate available sweeped data.
        aggregate(&parsed_vec, &mut full_data);
        // println!("{:?}", full_data);

        // STEP 3: Extract data to get first and last occurrence of each set.
        extract(full_data.clone(), &mut extracted_data);
        // println!("{:?}", extracted_data);
    }
    // STEP 4: Generate transition point between each set.
    let transition_pt_data = transition_pt(extracted_data.clone());
    println!("{:?}", transition_pt_data);

    // STEP 5: Generate Differential Nonlinearity (DNL).
    let dnl = dnl_gen(transition_pt_data.clone());
    println!("{:?}", dnl);

    // STEP 6: Generate Integral Nonlinearity (INL).
    let inl = inl_gen(dnl.clone());
    println!("{:?}", inl);
}
