use hlc::Timestamp;
use merkle::MerkleTree;

use std::fs::File;
use std::io::Write;

fn main() {
    let mut tree1 = MerkleTree::new();
    let mut tree2 = MerkleTree::new();

    let mut tree1_avg = 0;
    let mut tree2_avg = 0;
    // Generate 1000 identical timestamps for both trees
    for i in 0..1000 {
        let timestamp = Timestamp::new(
            i as u64,
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            format!("node_{}", i),
        );

        let ts_string = timestamp.to_string();

        // Measure time for populating trees
        let tree1_start = std::time::Instant::now();
        tree1.add_leaf(&ts_string);
        let tree1_duration = tree1_start.elapsed();
        // println!("{} Time to populate tree1: {:?}", i, tree1_duration);

        tree1_avg += tree1_duration.as_millis();
        tree1_avg /= 2;

        let tree2_start = std::time::Instant::now();
        tree2.add_leaf(&ts_string);
        let tree2_duration = tree2_start.elapsed();
        // println!("{} Time to populate tree2: {:?}", i, tree2_duration);

        tree2_avg += tree2_duration.as_millis();
        tree2_avg /= 2;
    }

    // Find and print differences (should be empty)
    let differences = tree1.find_differences(&tree2);
    if differences.is_empty() {
        println!("No differences found between trees");
    } else {
        println!("Unexpected differences found!");
        for (index, node1, node2) in differences {
            println!("Difference at index {}", index);
            println!("Tree 1 value: {}", node1.value);
            println!("Tree 2 value: {}", node2.value);
            println!("---");
        }
    }

    println!("Average time to populate tree1: {}ms", tree1_avg);
    println!("Average time to populate tree2: {}ms", tree2_avg);

    let bin1 = tree1.to_proto();
    let bin2 = tree2.to_proto();

    // Write protobuf data to files
    let proto_write_start = std::time::Instant::now();

    let mut file1 = File::create("tree1.pb").unwrap();
    file1.write_all(&bin1).unwrap();
    println!("Successfully wrote tree1 to protobuf file");

    let mut file2 = File::create("tree2.pb").unwrap();
    file2.write_all(&bin2).unwrap();
    println!("Successfully wrote tree2 to protobuf file");

    let proto_write_duration = proto_write_start.elapsed();
    println!(
        "Time to write trees to protobuf files: {:?}",
        proto_write_duration
    );
    println!(
        "Protobuf sizes - tree1: {:.2} KB, tree2: {:.2} KB",
        bin1.len() as f64 / 1024.0,
        bin2.len() as f64 / 1024.0
    );

    println!("{:2}", tree1.root.unwrap().value)
}
