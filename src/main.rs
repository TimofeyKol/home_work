use num_bigint::BigInt;

fn main(){
    // 8 ** 2022
    let base = BigInt::from(8);
    let exponent = 2022;
    let result_1 = base.pow(exponent);
    //println!("{}^{}={}", base, exponent, result);
    // 2 ** 2020
    let base = BigInt::from(2);
    let exponent = 2020;
    let result_2 = base.pow(exponent);
    //println!("{}^{}={}", base, exponent, result);
    let a = result_1 + result_2 - 17;
    let c = format!("{:b}", a);
    let mut coin = 0;
    for f in c.split(""){
        let m = [f];
        for i in m{
            if i == "1"{
                coin += 1;
            }
        }
    }
    println!("{}",coin);
}
