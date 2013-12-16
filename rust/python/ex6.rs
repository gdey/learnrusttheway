use std::fmt;
fn main(){

   let x = format!("There are {:d} types of people.", 10);
   let binary = "binary";
   let do_not = "don't";
   let y = format!("Those who know {:s} and those who {:s}.", binary, do_not);

   println(x);
   println(y);

   println!( "I said: {:?}.",x );
   println!( "I also said: '{:s}'.",y);

   let hilarious = false;
   let joke_evaluation = "Isn't that joke so funny?! {}";
   // Don't know how to send in a dynamic string to be formated, yet.
   println(format!("Isn't that joke so funny?! {}",hilarious));
   println("Isn't that joke so funny?! "+ if hilarious { "true" } else { "false" });
   //format_args!(fmt::format, joke_evaluation, hilarious.to_str);
   let w = "This is the left side of...";
   let e = "a string with a right side";
   println( w + e );


}
