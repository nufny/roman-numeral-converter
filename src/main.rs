use std::io;

fn main() {
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    //roman_to_dec(String::from(guess.trim()));
    dec_to_roman(guess.trim().parse().expect("Not a number"));


}
/*
I 	1
V 	5
X 	10
L 	50
C 	100
D 	500
M 	1 000
*/

fn roman_to_dec(roman: String){
    let letters: Vec<char> = roman.chars().collect();
    let mut output: u32=0;

    let mut index = 0;
    while &index < &letters.len(){
        output += match &letters.get(index).expect("") {
            'M' => 1000,
            'D' => 500,
            'C' => {
                index +=1;
                match &letters.get(index).expect("") {
                'M' => 900,
                'D' => 400,
                _ => {
                    index -=1;
                    100
                }
            }},
            'L' => 50,
            'X' => {
                index +=1;
                match &letters.get(index).expect("") {
                'C' => 90,
                'L' => 40,
                _ => {
                    index -=1;
                    10
                }
                }
            }
            'V' => 5,
            'I' => 1,
            _ => panic!()
        };
        index += 1;
    };

    println!("{}", output)


}

fn dec_to_roman(mut num: u32){
    let mut output = String::new();
    while num != 0{
        if num >= 1000{
            num -= 1000;
            output.push('M');
        } else if num >= 900 {
            num -= 900;
            output.push('C');
            output.push('M');
        } else if num >= 500 {
            num -= 500;
            output.push('D');
        } else if num >= 400 {
            num -= 400;
            output.push('C');
            output.push('D');
        } else if num >= 100 {
            num -= 100;
            output.push('C');
        } else if num >= 90 {
            num -= 90;
            output.push('X');
            output.push('C');
        } else if num >= 50 {
            num -= 50;
            output.push('L');
        }  else if num >= 40 {
            num -= 40;
            output.push('X');
            output.push('L');
        } else if num >= 10 {
            num -= 10;
            output.push('X');
        } else if num >= 9 {
            num -= 9;
            output.push('I');
            output.push('X');
        } else if num >= 5 {
            num -= 5;
            output.push('V');
        } else if num >= 4 {
            num -= 4;
            output.push('I');
            output.push('V');
        } else {
            num -= 1;
            output.push('I');
        }
    }
    println!("{}",output);

}