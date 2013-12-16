fn main(){
  
   println("I will now count my chickens:");
     /* Here we introduce the fmt! macro. In rust you can not just append
      * a number to a string; since rust is staticly type, you have to some
      * how convert the number to a string. There are two way we could do this,
      * (1) Use to_str, the other is to use (2)fmt! with a format stirng – like we
      *    have done.
      */  
   println!("Hens {:d}", (25 + 36 / 6));
   println!("Roosters {:d}", (100 - 25 * 3 % 4));
   println!("Now I will count the eggs: {:d}",
        3 + 4 + 1 - 5 + 4 % 4 - 1 /4 + 6 );
   let ans = if 3 + 2 < 5 - 7 { "yes" } else { "no" } ;
   println(fmt!( "Is it true that 3 + 2 < 5 - 7? %s",ans));
   println(format!( "Is it true that 3 + 2 < 5 - 7? {:s}",ans));
   println!( "Is it true that 3 + 2 < 5 - 7? {:s}",ans);
   println(fmt!( "Is it true that 3 + 2 < 5 - 7? %s",
    if 3 + 2 < 5 - 7 { "yes" } else {"no"} ));

   println!("What is 3 + 2? {:d}", 3 + 2);
   println!("What is 5 - 7? {:d}", 5 - 7);

   println("Oh, that's why it's False.");
   println("How about some more.");
   println!("Is it greater? {:s}", if 5 > -2 { "true" } else { "false" });
   println!("Is it greater or equal? {:s}", if 5 >= -2 { "true" } else { "false"});
   println!("Is it less or equal? {:s}", if 5 <= -2 { "true" }  else { "false" });
}
