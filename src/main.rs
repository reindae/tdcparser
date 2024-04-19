use std::fs::read;
use psfparser::binary::parse;
use psfparser::analysis::transient::TransientData;
use rust_xlsxwriter::worksheet::Worksheet;
use rust_xlsxwriter::workbook::Workbook;

fn main() {
    let mut workbook = Workbook::new();
    let mut worksheet = Worksheet::new();
    let lin_sweep = 2500;

    for sweep_count in 0..lin_sweep+1 {
        let sweep_count_format = format!("{:03}", sweep_count);
        // println!("Linear Sweep {sweep_count_format}");
        let path = format!("/tools/scratch/dwzhang/tdc_sky130_macros/tdc_64/tdc_64_sim.raw/sweepDelay-{sweep_count_format}_stepResponse.tran.tran");
        let read_bytes = read(path).unwrap();
        let parse_bytes = parse(&read_bytes).unwrap();
        let trans_data = TransientData::from_binary(parse_bytes);
        
        let delay: f64 = sweep_count as f64 * (7.0 / lin_sweep as f64);
        let mut sum = 0;
        worksheet.write_string(0, 0, "ΔDelay").unwrap();
        worksheet.write_number(0, sweep_count+1, delay).unwrap();

        // for (c, v) in trans_data.signals.iter() {
        //     println!("{}", c);
        // }

        for i in 0..252 {
            let name = format!("dout{i}");
            let dout_i: &Vec<f64> = &trans_data.signals[&name];
            let final_value = *(dout_i.last().unwrap());
            let final_value_i32: i32 = if final_value > 1.7 {
                1
            } else if final_value < 0.1 {
                0
            } else {
                panic!("dout was not close to either 0 or 1");
            };
            sum += final_value_i32;
            worksheet.write_string(i+1, 0, name).unwrap();
            worksheet.write_number(i+1, sweep_count+1, final_value_i32).unwrap();
            // println!("dout{i} = {final_value_i32}");
        }
        worksheet.write_string(253, 0, "total outputs");
        worksheet.write_number(253, sweep_count+1, sum).unwrap();
    }
    workbook.push_worksheet(worksheet);
    workbook.save("/tools/scratch/dwzhang/tdc_sky130_macros/tdc_64/tdc_sweep_data.xlsx");
}
