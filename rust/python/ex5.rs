fn main(){

   let my_name   = "Gautam Dey";
   let my_age    = 35; // Not a lie.
   let my_height = 74; // inches
   let my_weight = 140; // lbs
   let my_eyes   = "Brown";
   let my_teeth  = "White";
   let my_hair   = "Black";

   println!( "Let's talk about {:s}.", my_name );
   println!( "He's {:d} inches tall.", my_height );
   println!( "He's {:d} pounds heavy.", my_weight );
   println!( "Actually that's not too heavy.");
   println!( "He's got {:s} eyes and {:s} hair.", my_eyes, my_hair );
   println!( "His teeth are usually {:s} depending on the Red Bull.", my_teeth );
   println!( "If I add {:d}, {:d}, and {:d} I get {:d}.",
       my_age, my_height, my_weight, my_age + my_height + my_weight );
}
