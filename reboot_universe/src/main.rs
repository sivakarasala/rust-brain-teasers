use std::f32::EPSILON;

fn main() {
   if 0.1 + 0.2 == 0.3 {
    println!("Arithmetic still works.");
   } else {
    println!("Please reboot the universe.");
   }

   if (0.1f32 + 0.2f32 - 0.3f32).abs() < EPSILON {
    println!("Arithmetic works");
   }
}
