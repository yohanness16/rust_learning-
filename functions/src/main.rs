fn main() {
    my_first_function();
    my_second_function();
    opening_shop("burayu" ,  7);
    let y =sum_and_square(5, 6, 7);
    println!("the result is {y}")
}



// creating a function 

fn my_first_function(){
    println!("hi , this is my first rust function");
}

fn my_second_function(){
    println!("hi , this is my second rust function");
}

fn opening_shop(location : &str , time : i32){
    println!("i am opening my shop in {} at {} oclock  " , location , time);
}

fn sum_and_square(x : i32 , y : i32  , z : u32) -> i32 {
    let mut sum = x + y;
    let square = {
        let mut  multiplayer = z;
        loop {
            sum=sum*sum;
            if multiplayer >=1 {
                multiplayer=multiplayer-1;

            }
            else {
                break;
            }
            

        }
        sum
        

    };

    square
}