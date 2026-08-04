



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

    let age = 45;

  // the match keyword 
  // is used to match a possiblity of results for an expression 
  // the syntax is 
  // match expression {
  //  arm =>{}
  // }
  
 let value = match age {
    
  /*  0..20 => true,
   20..40 => false,
   40..60 => true,
   _ => false,  // in match cases if we want to refer all other cases we can use the underscore '_' as an arm and assign a fallback value 

*/  // other solutiuon 
    input if input < 20 => true,
    input if input > 20 && input < 40 => false,
    input if input > 40 && input < 60 => true,
    _ => false,


  }; 
  // we can also assign a match expression to a variable and use it later in the code
  println!("hi it is {value}");


  // example 2 even or odd with match statement

  let x = 4;

  let results = match x{
    y if y % 2 == 0 => "even" ,
    y if y % 2 == 1 => "odd",
    _ => unreachable!(),  // unreachable is a macro in rust used to define when the result cant be happen 

    
  };

  print!("The number is {results}");
} 

fn even_or_odd(num:i32){
    let result = if num % 2 == 0 { "even"} else { "odd" };
     println!( "it is {result} number");
}