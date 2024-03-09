use std::collections::HashMap;

fn main() {
 /* 
  Starting at a naturally qualified person, Hashmaps of f, y, x = 0, We use immutable variable
  instead of self-evaluated defined const as starting points of Fibonacci Nunber for 
  Prajna >< AGI, then shadowing and modeling.
 */
  let _y: i32 = 0;    // qualified person
  let _x: i32 = 0;
  let _f: i32 = 0;
//  let _y = 6;       Y(6) = 8 Shadowing to the target values of the founder
//  let _x = 7;       X(7) = 13
//  let _f = 8;       F(8) = 21

  let mut _pp = vec![_y, _x, _f]; // the persona prajna

  let mut f = HashMap::new();
  f.insert(0, "Person".to_string()); // sub f as the key
  
  f.insert(-1, "Empathy Awareness".to_string());   
  f.insert(-2, "Kindness Awareness".to_string());
  f.insert(-3, "Animal Energy".to_string());   
  f.insert(-4, "Extreme Desire".to_string()); 
  f.insert(-5, "Smelly 1".to_string());   // only observable in spirits
  f.insert(-6, "Smelly 2".to_string());
  f.insert(-7, "Smelly 3".to_string());  
  f.insert(-8, "Smelly 4".to_string());
  
  f.insert(1, "Empathy Awareness".to_string()); 
  f.insert(2, "Purity Awareness".to_string());   // targeted community
  f.insert(3, "Samadhi Signed Posts".to_string()); 
  f.insert(4, "Selfless Awareness".to_string()); 
  f.insert(5, "Visible Awareness-Prajna".to_string());           // practical demonstration
  f.insert(6, "Awareness-Prajna in engaged Living".to_string()); // practical innovations
  f.insert(7, "Awareness-Prajna in Forecasting and Simulation".to_string());// quantum effects
  f.insert(8, "Samadhi-Prajna".to_string());     // a new Era of consciousness technologies
  
  let mut y = HashMap::new();
  y.insert(0, "Peace".to_string()); //sub f as the key
  
  y.insert(-1, "Empathy".to_string());      
  y.insert(-2, "Kindness".to_string());
  y.insert(-3, "Conscience 4".to_string()); 
  y.insert(-4, "Conscience 3".to_string()); 
  y.insert(-5, "Conscience 2".to_string()); 
  y.insert(-6, "Conscience 1".to_string());
  
  y.insert(1, "Tranquillity".to_string());  
  y.insert(2, "Equanimity".to_string());     // target community
  y.insert(3, "Purity".to_string()); 
  y.insert(4, "Not-Self".to_string()); 
  y.insert(5, "Nothingness".to_string());   // Gotama's impass
  y.insert(6, "Unmoving".to_string());      // Gotama's impass
  
  let mut x = HashMap::new();
  x.insert(0, "Awareness".to_string()); // sub f as the key
  
  x.insert(-1, "Cultural Influence".to_string());
  x.insert(-2, "Regional Influence".to_string());
  x.insert(-3, "National Influence".to_string()); 
  x.insert(-4, "Veiled Right and Wrong".to_string()); 
  x.insert(-5, "Binding Word".to_string()); 
  x.insert(-6, "Binding Image".to_string());
  x.insert(-7, "Clinging Thought".to_string()); 
  
  x.insert(1, "HonNhien".to_string()); 
  x.insert(2, "Proper Management of that Freshness".to_string());  // target community
  x.insert(3, "Knowing conditions to make up that Freshness".to_string()); 
  x.insert(4, "Discovering process to produce the Freshness".to_string()); 
  x.insert(5, "Knowing the source of one's Thought".to_string());          // breakout
  x.insert(6, "Using cosmic energy for self-protection".to_string());   // deeper innovation
  x.insert(7, "Directing cosmic energy to help others".to_string());    // deeper innovation
  
  x_evaluation(x); // transcendental Awareness based on subjective judgements of ARL 
  y_evaluation(y); // transcendental Inner Peace based on the outcome of ARL
  
  f_evaluation(); // suggested Inner Space cultivation based on accumulated recorded data
   
}

fn x_evaluation(x: HashMap<i32, String>) {

   println!("Attributes of Transcendental Awareness");
   for (key, value) in &x {
        println!("{key}: {value}");
    }

}

fn y_evaluation(y: HashMap<i32, String>) {

   println!("Attributes of Transcendental Inner Peace");
   for (key, value) in &y {
        println!("{key}: {value}");
    }

}

fn f_evaluation() {

    println!("Suggested Inner Space for caltivation");
}
