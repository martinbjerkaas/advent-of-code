use std::{clone, cmp::{max, min}, collections::{BTreeMap, HashMap, HashSet}, fs, path::Path};
//use aoc::*;

const INPUT_FILE_PATH: &'static str = "data/2025-8.txt";

#[derive(Debug,Hash,Clone, Copy,PartialEq, Eq, PartialOrd, Ord)]
struct Point3D {
    x: i64,
    y: i64,
    z: i64
}

impl Point3D {
    fn new_from_string(input : &str) -> Point3D {
        let mut values = input.trim().split_terminator(",");
        Self { 
            x: values.next().expect("Could not map X from string").parse::<i64>().expect("Could not parse X to u32"), 
            y: values.next().expect("Could not map X from string").parse::<i64>().expect("Could not parse X to u32"),
            z: values.next().expect("Could not map X from string").parse::<i64>().expect("Could not parse X to u32"), 
        }
    }

    fn distance_to_point(&self, other: &Point3D) -> u64 {
        // rot(kat2+kat2+kat2)
        let distance: u64 = ((other.x - self.x).pow(2) + (other.y - self.y).pow(2)+ (other.z - self.z).pow(2)) as u64 ;
        return distance as u64;
    }
}

fn first_part(inputstring: &str) -> u32{
    let junction_boxes: Vec<Point3D> = inputstring.lines().map(| juntion_box |Point3D::new_from_string(juntion_box)).collect();
    let mut pairs: Vec<(u64,usize,usize)> = Vec::new();
    let mut circuits: Vec<HashSet<u32>> = Vec::new();

    for point_index in 0..junction_boxes.len() {
        for index in point_index+1..junction_boxes.len() {
            let p1 = junction_boxes[point_index];
            let p2 = junction_boxes[index];
            
            pairs.push((p1.distance_to_point(&p2), point_index,index));
        }
    }
    pairs.sort();

    // connecter kun på de 1000 korteste
    let pairs_slice: Vec<HashSet<u32>> = pairs[0..1000].iter().map(|(_,p1,p2)| HashSet::from([*p1 as u32,*p2 as u32])).collect();

    println!("\nSlices: {:?}\n", pairs_slice);

    circuits.extend(pairs_slice.clone());

    // antall loops. 1000 i faktisk inputfil
   'merge_loop: for merge_count in 0..1000 {
        for circuit_index in 0..circuits.len() {
            for index in 0..circuits.len() {
                let circuit_to_check = circuits[index].clone();
                //println!("Checking: {:?} against {:?}", circuits[circuit_index], circuit_to_check);
                if !circuits[circuit_index].is_disjoint(&circuit_to_check) && circuits[circuit_index] != circuit_to_check {
                    circuits[circuit_index].extend(circuit_to_check);
                    circuits.remove(index);
                    //println!("Breaking merge loop");
                    continue 'merge_loop;
                }
            }
        }
    }



    println!("\nResult::");
    println!("{:?}", circuits);

/*     for circuit in &circuits {
        println!("Circuit:: {:?}", circuit);
        println!("{:?}", circuit.iter().for_each(|x| println!("{:?}", &junction_boxes[*x as usize])));
    } */

    circuits.sort_by(|b,a| a.len().cmp(&b.len()));

    let top_three = &circuits[0..3];
    let result: u32 = top_three.iter().map(|x|x.len() as u32).product();

    println!(" Top three: {:?}", top_three);
    

    return result;

    
    // 2184 to
}

fn second_part(inputstring: &str) -> u64 {
    let junction_boxes: Vec<Point3D> = inputstring.lines().map(| juntion_box |Point3D::new_from_string(juntion_box)).collect();
    let mut pairs: Vec<(u64,usize,usize)> = Vec::new();
    let mut circuits: Vec<HashSet<u32>> = Vec::new();

    for point_index in 0..junction_boxes.len() {
        for index in point_index+1..junction_boxes.len() {
            let p1 = junction_boxes[point_index];
            let p2 = junction_boxes[index];
            
            pairs.push((p1.distance_to_point(&p2), point_index,index));
        }
    }
    pairs.sort();
    for (distance, p1,p2) in &pairs {
       println!("Distance: {:?} p1:{:?} p2:{:?}", distance, junction_boxes[*p1], junction_boxes[*p2]);
    }
    

    // connecter kun på de 1000 korteste
    let pairs_slice: Vec<HashSet<u32>> = pairs.iter().map(|(_,p1,p2)| HashSet::from([*p1 as u32,*p2 as u32])).collect();


    println!("\nSlices: {:?}\n", pairs_slice);

    circuits.extend(pairs_slice.clone());
    let mut last_merge:HashSet<u32> = HashSet::new(); 

        // antall loops. 1000 i faktisk inputfil
   'merge_loop: for merge_count in 0..1000 {
        for circuit_index in 0..circuits.len() {
            for index in 0..circuits.len() {
                let circuit_to_check = circuits[index].clone();
                //println!("Checking: {:?} against {:?}", circuits[circuit_index], circuit_to_check);
                if !circuits[circuit_index].is_disjoint(&circuit_to_check) && circuits[circuit_index] != circuit_to_check {
                    last_merge = circuit_to_check.clone();
                    circuits[circuit_index].extend(circuit_to_check);
                    circuits.remove(index);
                    //println!("Breaking merge loop");
                    println!("Merging: {:?}", last_merge);
                    
                    continue 'merge_loop;
                }
            }
        }
    }

    for merge in last_merge {
        println!("Last merge: {:?}", &junction_boxes[merge as usize]);
    }

    println!("\nResult::");
    println!("{:?}", circuits);

/*     for circuit in &circuits {
        println!("Circuit:: {:?}", circuit);
        println!("{:?}", circuit.iter().for_each(|x| println!("{:?}", &junction_boxes[*x as usize])));
    } */
    
    return 0;
}
fn main(){
    let test_string = "162,817,812\n57,618,57\n906,360,560\n592,479,940\n352,342,300\n466,668,158\n542,29,236\n431,825,988\n739,650,466\n52,470,668\n216,146,977\n819,987,18\n117,168,530\n805,96,715\n346,949,466\n970,615,88\n941,993,340\n862,61,35\n984,92,344\n425,690,689";

    //println!("First part example: {:?}", first_part(test_string));
    println!("Second part example: {:?}", second_part(test_string));

    match fs::read_to_string(Path::new(INPUT_FILE_PATH)) {
        Ok(file_content) => {
            //println!("First part file input: {:?}", first_part(&file_content));
            //println!("Second part file input: {:?}", second_part(&file_content));
        },
        Err(err) => eprintln!("Could not find inputfile with error: {err}")
    }



}

/* #[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn first_part_is_correct() {
        let file_content = fs::read_to_string(Path::new(INPUT_FILE_PATH)).expect("Could not load file");
        assert_eq!(first_part(&file_content), 15873079081);
    }

    #[test]
    fn second_part_is_correct() {
        let file_content = fs::read_to_string(Path::new(INPUT_FILE_PATH)).expect("Could not load file");
        assert_eq!(second_part(&file_content), 22617871034);
}

} */