fn main() {
   let cars = 100;
   let space_in_a_car = 4.0;
   let drivers = 30;
   let passengers = 90;
   let cars_not_driven = cars - drivers;
   let cars_driven = drivers;
   let carpool_capacity = cars_driven * space_in_a_car as int;
   let average_passengers_per_car = passengers / cars_driven;

   println!( "There are {:d} cars available.",cars);
   println!( "There are only {:d} drivers available.", drivers);
   println!( "There will be {:d} empty cars today.",cars_not_driven);
   println!( "We can transport {:d} people today.", carpool_capacity );
   println!( "We have {:d} to carpool today.", passengers );
   println!( "We need to put about {:d} in each car.", average_passengers_per_car );

}
