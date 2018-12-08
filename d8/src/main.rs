use std::fs;

struct Node {
    child_quantity: i32,
    children: Vec<Node>,
    metadata_quantity: i32,
    metadata: Vec<i32>
}

fn main() {
    let small = false;

    let filename: &str;
    if small {
        filename = "input_small.txt";
    } else {
        filename = "input.txt";
    }

    let mut numbers = read_inputs(filename);

    let root = create_node(&mut numbers);
    print_data(&root, 0);

    let sum = analyze_sum_metadata(&root);

    println!("Sum of metadata is: {}", sum);
}

fn read_inputs(filename: &str) -> Vec<i32> {
    let file_contents = fs::read_to_string(filename).expect("Error in reading file");

    let mut numbers: Vec<i32> = file_contents.split(" ").map(|s| s.parse::<i32>().unwrap()).collect();
    numbers.reverse();

    return numbers;
}

fn create_node(numbers: &mut Vec<i32>) -> Node {
    let num_children = numbers.pop().unwrap();

    let num_metadata = numbers.pop().unwrap();

    let mut children = Vec::new();
    for _i in 0..num_children {
        children.push(create_node(numbers));
    }

    let mut metadata = Vec::new();
    for _i in 0..num_metadata {
        metadata.push(numbers.pop().unwrap());
    }

    return Node{child_quantity: num_children, metadata_quantity: num_metadata, children: children, metadata: metadata};
}

fn analyze_sum_metadata(node: &Node) -> i32 {
    let mut ret = 0;

    for md in &node.metadata {
        ret += md;
    }

    for child in &node.children {
        ret += analyze_sum_metadata(&child);
    }

    return ret;
}

fn print_data(node: &Node, num_tabs: i32) {
    for _i in 0..num_tabs {
        print!("\t");
    }

    println!("# children: {}, # metadata: {},    metadata: {:?}", node.child_quantity, node.metadata_quantity, node.metadata);

    for child in &node.children {
        print_data(&child, num_tabs + 1);
    }
}
