use std::fs::read;
use psfparser::binary::parse;
use psfparser::analysis::transient::TransientData;
use rust_xlsxwriter::worksheet::Worksheet;
use rust_xlsxwriter::workbook::Workbook;


fn main() {
    let mut workbook = Workbook::new();
    let mut worksheet = Worksheet::new();
    let mut first_occur = 0;
    let mut last_occur = 252;
    let mut sum = 0;
    let mut lin_sweep = 2500;

    // worksheet.write_string(0, 0, "ΔDelay").unwrap();
    // worksheet.write_string(253, 0, "total outputs");

    // Forward sweep
    for sweep_count in 0..lin_sweep+1 {
        let sweep_count_format = format!("{:03}", sweep_count);
        // println!("Linear Sweep {sweep_count_format}");
        let path = format!("/tools/scratch/dwzhang/tdc_sky130_macros/tdc_64/tdc_64_sim.raw/sweepDelay-{sweep_count_format}_stepResponse.tran.tran");
        let read_bytes = read(path).unwrap();
        let parse_bytes = parse(&read_bytes).unwrap();
        let trans_data = TransientData::from_binary(parse_bytes);
        
        let delay: f64 = sweep_count as f64 * (7.0 / lin_sweep as f64);

        // for (c, v) in trans_data.signals.iter() {
        //     println!("{}", c);
        // }
        for i in 0..252 {
            let name = format!("dout{i}");
            let dout_i: &Vec<f64> = &trans_data.signals[&name];
            let final_value = *(dout_i.last().unwrap());
            let final_value_i32: u16 = if final_value > 1.7 {
                1
            } else if final_value < 0.1 {
                0
            } else {
                panic!("dout was not close to either 0 or 1");
            };
            sum += final_value_i32;
            // worksheet.write_string(i+1, 0, name).unwrap();
            // worksheet.write_number(i+1, first_occur+1, final_value_i32).unwrap();
            // println!("dout{i} = {final_value_i32}");
        }
        if sum == first_occur {
            worksheet.write_number(0, first_occur*2, delay).unwrap();
            worksheet.write_number(1, first_occur*2, sum).unwrap();
            first_occur = first_occur + 1;
        }
        sum = 0;
    }

    // Backward sweep
    for sweep_count in (0..lin_sweep+1).rev() {
        let sweep_count_format = format!("{:03}", sweep_count);
        let path = format!("/tools/scratch/dwzhang/tdc_sky130_macros/tdc_64/tdc_64_sim.raw/sweepDelay-{sweep_count_format}_stepResponse.tran.tran");
        let read_bytes = read(path).unwrap();
        let parse_bytes = parse(&read_bytes).unwrap();
        let trans_data = TransientData::from_binary(parse_bytes);
        
        let delay: f64 = sweep_count as f64 * (7.0 / lin_sweep as f64);

        for i in 0..252 {
            let name = format!("dout{i}");
            let dout_i: &Vec<f64> = &trans_data.signals[&name];
            let final_value = *(dout_i.last().unwrap());
            let final_value_i32: u16 = if final_value > 1.7 {
                1
            } else if final_value < 0.1 {
                0
            } else {
                panic!("dout was not close to either 0 or 1");
            };
            sum += final_value_i32;
        }
        if sum == last_occur {
            worksheet.write_number(0, last_occur*2+1, delay).unwrap();
            worksheet.write_number(1, last_occur*2+1, sum).unwrap();
            last_occur = if last_occur > 0 {
                last_occur - 1
            }
            else {
                252
            }
        }
        sum = 0;
    }

    workbook.push_worksheet(worksheet);
    workbook.save("/tools/scratch/dwzhang/tdc_sky130_macros/tdc_64/tdc_sweep_data.xlsx");
}
