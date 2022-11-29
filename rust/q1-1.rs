fn main () {
    let mut count = 11;
    loop {
        let num_2 = format!("{:b}", count);
        let num_8 = format!("{:o}", count);
        let num_10 = format!("{}", count);

        println!("{}",num_2);
        println!("{}",num_2.chars().rev().collect::<String>());
        println!("{}",num_8);
        println!("{}",num_8.chars().rev().collect::<String>());
        println!("{}",num_10);
        println!("{}",num_10.chars().rev().collect::<String>());

        if num_2 == num_2.chars().rev().collect::<String>() &&
            num_8 == num_8.chars().rev().collect::<String>() &&
            num_10 == num_10.chars().rev().collect::<String>() {
                println!("{}",count);
                break;
            }
        count += 2;
    }
}
