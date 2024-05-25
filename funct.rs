fn main(){
    println!("1st function");
    
    new_fn(334);
    second_function(7,'H');
    exp();
    
    let nigg = rv();
    println!("The value of nigg is - {}", nigg)
    
}

fn new_fn(x : i32){
    println!("The value of x is {}", x)
}

fn second_function(x : i32 , y : char){
    println!("The value of x is {} , and the value of y is {}", x , y)
}

fn exp(){
    let y = {
        let x = 9;
        x + 1
    };
    println!("The value of y is {}", y)
}

fn rv() -> i32 {
    9-3
}
