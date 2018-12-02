use std::fs;

fn main() {
    let file_contents = fs::read_to_string("input.txt").expect("Error in reading file");

    let words: Vec<&str> = file_contents.split("\n").collect();

//    let mut two_same = 0;
//    let mut three_same = 0;

    let num_words = words.len();

    for i in 0..num_words {
        for j in i..num_words {
            if i == j {
                continue;
            }
            let word1 = words[i];
            let word2 = words[j];

            let mut num_differences = 0;

            for position in 0..word1.len() {

                let c1 = word1.as_bytes()[position];
                let c2 = word2.as_bytes()[position];
                if c1 != c2 {
                    num_differences = num_differences + 1;
                }
            }


            if num_differences == 1 {
                println!("Hittat en skillnad {} och {}", word1, word2);

                for position in 0..word1.len() {

                    let c1 = word1.as_bytes()[position];
                    let c2 = word2.as_bytes()[position];
                    if c1 == c2 {
                        print!("{}", c1 as char);
                    }
                }
                println!("");
                panic!("Klar");
            }
            println!("i {}\tj {}\tword1 {}\tword2 {}\tnum_differences {}", i, j, word1, word2, num_differences);
        }
    }
}
