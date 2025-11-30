use std::{fs, ops::{Range}, path::Path};
#[derive(Debug,Clone)]
struct Report {
    levels: Vec<i32>,
    correction_index: usize,
    errors: Vec<i32>,
    evaluations: u8,
    finished: bool
}

impl Report {
    fn new(levels: Vec<i32>) -> Report {
        Self { 
            levels, 
            correction_index: 0,
            errors: Vec::new(), 
            evaluations: 0,
            finished: false
        }
    }

    fn evaluate(&mut self){
        // 490 safe 510 unsafe
        let accepted_range: Range<i32>= -3..4;
        let mut last_num: Option<i32> = None;
        let mut last_diff: Option<i32> = None;
        let mut level_copy = self.levels.clone();
        let level_copy_length = level_copy.len();

        if !self.errors.is_empty() {
            level_copy.remove(self.correction_index);
            self.correction_index +=1;
        }

        let corrected_levels = level_copy.clone();

        self.errors.clear();

        for (_index, num) in level_copy.iter_mut().enumerate(){
            if last_num.is_none() {
                let _ = last_num.insert(*num);
                continue;
            }

            let diff: i32 = *num - last_num.unwrap();

            if last_diff.is_none() {
                let _ = last_diff.insert(diff);
            }

            //println!("Checking: {:?}", &self.levels);
            //println!("num: {}, last_num: {:?}, diff: {:?}, last_diff: {:?}, direction_check:", num, last_num, diff, last_diff);

            // checks direction and if in range
            if (last_diff.unwrap() * diff) < 1 || !accepted_range.contains(&diff) {
                println!("Failed check: {:?}, Correction index: {}", &corrected_levels, &self.correction_index);
                //println!("num: {}, last_num: {:?}, diff: {:?}, last_diff: {:?}, direction_check: {:?}", num, last_num, diff, last_diff, (last_diff.unwrap()*diff) >=0);
                self.errors.push(1);
                break;                

            }

            let _ = last_num.insert(*num);
            let _ = last_diff.insert(diff);

        }

        if self.correction_index >= level_copy_length {
            self.finished = true;
        }

        if self.errors.is_empty() {
            self.finished = true;
        }

        println!("Evaluated: {:?}. Errors: {:?}. Finished: {}", &corrected_levels, &self.errors, &self.finished);
        self.evaluations += 1;
    }
}

 fn main(){
/*     let mut reports: Vec<Vec<i32>> = Vec::new();
    reports.push(vec![7,6,4,2,1]);
    reports.push(vec![1,2,7,8,9]);
    reports.push(vec![9,7,6,2,1]);
    reports.push(vec![1,3,2,4,5]);
    reports.push(vec![8,6,4,4,1]);
    reports.push(vec![1,3,6,7,9]); */

    let mut reports: Vec<Report> = Vec::new();
    let mut safe_reports: Vec<Report> = Vec::new();
    let mut unsafe_reports: Vec<Report> = Vec::new();

    let file_content = fs::read_to_string(Path::new("data/day2.txt")).expect("Could not load file");
    for report_string in file_content.split("\n") {
        //println!("{}", &report);

        let mut levels_to_push: Vec<i32> = Vec::new();
        for num in report_string.split_whitespace() {
            levels_to_push.push(num.parse::<i32>().expect("Could not parse str to i32"));
        }

        reports.push(Report::new(levels_to_push));
    };

    for (_index, report) in reports.iter_mut().enumerate() {

/*         if index > 10 {
            break;
        } */

        while !report.finished {
            report.evaluate();
        }
        println!("------------------------- Finished report -------------------------");

        if report.errors.is_empty() {
            safe_reports.push(report.clone());
        } else {
            unsafe_reports.push(report.clone());
        }
    }

    println!("Safe: {}, Unsafe:{}", &safe_reports.len(), &unsafe_reports.len());
    //println!("{:?}", unsafe_reports)


/*     safe_reports = 0;
    unsafe_reports = 0;

    for report in reports.iter_mut() {
        report.evaluate();

        if report.evaluated && report.errors.is_empty() {
            safe_reports += 1;
        } else {
            unsafe_reports += 1;
        }
    }

    println!("Safe: {}, Unsafe:{}", &safe_reports, &unsafe_reports); */

 }
