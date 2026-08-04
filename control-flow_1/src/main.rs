

fn main (){
    /* 
    /// conditional statements 
    /// are statements which have a branch operation there ar some tasks and happens if some condition is happend
    /// they will define that if some thing is this then it will be this 
    /// 
    /// 
    /// the if else statement 
    /// 
    /// the syntax is 
    /// if true {
    ///   // thers is some task or action done if the expression is true 
    /// 
    /// }
    */
    if 1 == 2 {
        println!("hi");
    }
    else {
        println!("bye");
    }

  even_or_odd(19);
  
    
}

fn even_or_odd(num:i32){
    let result = if num % 2 == 0 { "even"} else { "odd" };
     println!( "it is {result} number");
}