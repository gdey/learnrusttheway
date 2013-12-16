fn x( string : ~str, number: int ) -> ~str{
   let mut myString = string.clone();
   for _i in range(1,number) {
        myString = myString + string.clone()
   };
   myString
}
fn main() {
   println("Mary had a little lamb.");
   println!("Its fleece was white as {}.","snow");
   println(x(~".",10));
   let end1 = "C";
   let end2 = "h";
   let end3 = "e";
   let end4 = "e";
   let end5 = "s";
   let end6 = "e"; 
   let end7 = "B";
   let end8 = "u";
   let end9 = "r";
   let end10 = "g";
   let end11 = "e";
   let end12 = "r";

   print(end1+end2+end3+end4+end5+end6+" ");
   println(end7+end8+end9+end10+end11+end12);
}
