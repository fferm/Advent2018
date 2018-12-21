fn main() {
    let mut r0: isize = 13443200;
    let mut r1: isize = 0;
    let mut r2: isize = 0;
    let mut r3: isize = 0;
    let mut r4: isize = 0;
    let mut r5: isize = 0;
    let mut r6: isize = 0;

    //  #ip 2
    loop {
        r4 = 123;            //  0   seti 123 0 4
        r4 = r4 & 456;            //  1   bani 4 456 4

        if r4 == 72 {                     //  2   eqri 4 72 4
            break;                      //  3   addr 4 2 2  // JMP
        }                                       //  4   seti 0 0 2  // JMP

    }

    println!("r1: {}, r2: {}, r3: {}, r4: {}, r5: {}, r6: {}", r1, r2, r3, r4, r5, r6);

    r4 = 0; //  5   seti 0 7 4

    loop {
        r3 = r4 | 65536; //  6   bori 4 65536 3
        r4 = 10283511;      //  7   seti 10283511 1 4

        loop {
            r1 = r3 & 255;      //  8   bani 3 255 1
            r4 = r4 + r1;            //  9   addr 4 1 4
            r4 = r4 & 16777215;     //  10  bani 4 16777215 4
            r4 = r4 * 65899;        //  11  muli 4 65899 4
            r4 = r4 & 16777215; //  12  bani 4 16777215 4

            if 256 > r3 {       //  13  gtir 256 3 1
                break;          //  14  addr 1 2 2  // JMP
            }                           //  15  addi 2 1 2  // JMP
            //  16  seti 27 8 2  // JMP

            r1 = 0;             //  17  seti 0 1 1

            loop {
                r5 = r1 + 1;        //  18  addi 1 1 5
                r5 = r5 * 256;      //  19  muli 5 256 5

                if r5 > r3 {            //  20  gtrr 5 3 5
                    //  21  addr 5 2 2  // JMP
                    //  22  addi 2 1 2  // JMP
                    break;//  23  seti 25 3 2  // JMP
                }
                r1 = r1 + 1;    //  24  addi 1 1 1
            }  //  25  seti 17 0 2  // JMP

            r3 = r1;        //  26  setr 1 4 3
        }//  27  seti 7 6 2  // JMP

        println!("r4 before break: {}", r4);
        if r4 == r0 {    //  28  eqrr 4 0 1
            break;              //  29  addr 1 2 2  // JMP
        }

    }//  30  seti 5 2 2  // JMP
}
