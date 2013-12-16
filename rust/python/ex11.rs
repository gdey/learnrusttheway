use std::io;


fn main() {
   let stdin = io::stdin();

   print( "How old are you? " );
   let age = stdin.read_line();
   
   print( "How tall are you? " );
   let height = stdin.read_line();
   print( "How much do you weigh? " );
   let weight = stdin.read_line();

   println!("So, you're {:s} years old, {:s} tall and {:s} heavy.",
             age, height, weight);

   
}
