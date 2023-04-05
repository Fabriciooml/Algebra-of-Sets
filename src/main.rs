use indexmap::set::IndexSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_sets_from_file() -> Vec<IndexSet<i32>> {
  let mut sets_vector: Vec<IndexSet<i32>> = Vec::new();
  let file = File::open("src/input.txt").expect("Failed to open file");
  
  for line in BufReader::new(file).lines() {
      sets_vector.push(read_set(line));
  }
  sets_vector
}

fn read_set(line: Result<String, std::io::Error>) -> IndexSet<i32> {
  let mut set = IndexSet::new();    

  let string = line.expect("Failed to read line");
      let string = string.replace(&['{', '}'][..], "");
      let tokens: Vec<&str> = string.split_whitespace().collect();
      for i in 2..tokens.len(){
          for value in tokens[i].split(',') {
              if let Ok(num) = value.trim().parse::<i32>() {
                  set.insert(num);
              }
          }
      } 
  set
}

fn belongs(set: IndexSet<i32>, value_to_compare: &i32) -> bool {
  for i in 0..set.len(){
      let value_on_position = set.get_index(i).expect("no value on position");
      if value_to_compare == value_on_position {
          return true;
      }
  }
  false
}

fn does_not_belong(set: IndexSet<i32>, value_to_compare: &i32) -> bool {
  !belongs(set, value_to_compare)
}

fn is_subset(set: IndexSet<i32>, set_to_compare: IndexSet<i32>) -> bool {
  let mut number_of_belongs = 0;
  for i in 0..set_to_compare.len(){
      let value_to_compare = set_to_compare.get_index(i).expect("no value on position");
      if belongs(set.clone(), value_to_compare) {
          number_of_belongs += 1;
      }
  }
  if number_of_belongs == set_to_compare.len() {
      return true;
  }
  false
}

fn is_not_subset(set: IndexSet<i32>, set_to_compare: IndexSet<i32>) -> bool {
  !is_subset(set, set_to_compare) 
}

fn is_proper_subset(set: IndexSet<i32>, set_to_compare: IndexSet<i32>) -> bool {
  if is_subset(set.clone(), set_to_compare.clone()) && set.len() > set_to_compare.len(){
      return true;
  } 
  false
}

fn is_not_proper_subset(set: IndexSet<i32>, set_to_compare: IndexSet<i32>) -> bool {
  !is_proper_subset(set, set_to_compare) 
}

fn union(mut set: IndexSet<i32>, mut set_to_union: IndexSet<i32>) -> IndexSet<i32> {
  for _i in 0..set_to_union.len(){
      let a = set_to_union.pop().expect("set is empty");
      set.insert(a);
  }
  set
}

fn intersection(set: IndexSet<i32>, set_to_intersect: IndexSet<i32>) -> IndexSet<i32> {
  let mut intersected:IndexSet<i32> = IndexSet::new();
  for i in 0..set.len(){
      for j in 0..set_to_intersect.len() {
          let value_first_set = *set.get_index(i).expect("no value on position");
          let value_second_set = *set_to_intersect.get_index(j).expect("no value on position");
          if value_first_set == value_second_set {
              intersected.insert(value_first_set);
          }
      }
  }
  intersected
}

fn cartesian_product(first_set: IndexSet<i32>, second_set: IndexSet<i32>) -> Vec<(i32, i32)> {
  let mut product:Vec<(i32, i32)> = Vec::new();
  for i in 0..first_set.len(){
    for j in 0..second_set.len() {
      let intermediate = (first_set[i], second_set[j]);
      product.push(intermediate);
    }
  }
  product
}

fn revert_cartesian_product(cartesian_product_vec: Vec<(i32, i32)>) -> Vec<IndexSet<i32>> {
  let mut set_a:IndexSet<i32> = IndexSet::new();
  let mut set_b:IndexSet<i32> = IndexSet::new();
  let mut sets_vec:Vec<IndexSet<i32>> = Vec::new();
  for i in 0..cartesian_product_vec.len() {
    set_a.insert(cartesian_product_vec[i].0);
    set_b.insert(cartesian_product_vec[i].1);
  }
  sets_vec.push(set_a);
  sets_vec.push(set_b);
  sets_vec
}

fn subtraction(set: IndexSet<i32>, set_to_subtract: IndexSet<i32>) -> IndexSet<i32> {
  let mut subtraction_set = set.clone();
  for i in 0..set_to_subtract.len() {
    subtraction_set.remove(&set_to_subtract[i]);
  }
  subtraction_set
}

fn power_set(set: IndexSet<i32>) -> Vec<IndexSet<i32>> {
  let mut result = Vec::new();
  result.push(IndexSet::new()); // add empty set
  for i in 0..set.len() {
    let elem = *set.get_index(i).expect("no element on index");
      let len = result.len();
      for j in 0..len {
          let mut subset = result[j].clone();
          subset.insert(elem);
          result.push(subset);
      }
  }
  result
}

fn revert_power_set(mut power_set_vec: Vec<IndexSet<i32>>) -> IndexSet<i32> {
  let reverted = power_set_vec.pop().expect("Vector is empty");
  reverted
}

fn testing(mut sets_vector: Vec<IndexSet<i32>>) {
  let second_set = sets_vector.clone().into_iter().nth(1).expect("no set on this position");
  let first_set = sets_vector.clone().into_iter().nth(0).expect("no set on this position");
  println!("Set: {:?}", sets_vector);
  let number_to_compare = 11;
  println!("{} belongs to the first set? {}", number_to_compare, belongs(first_set.clone(), &number_to_compare));
  println!("{} DON'T belongs to the first set? {}", number_to_compare, does_not_belong(first_set.clone(), &number_to_compare));
  println!("second is a subset of the first one? {}", is_subset(
      first_set.clone(),
      second_set.clone()
  ));
  println!("second set isn't a subset the first one? {}", is_not_subset(
      first_set.clone(),
      second_set.clone()
  ));
  println!("second is a proper subset of the first one? {}", is_proper_subset(
      first_set.clone(),
      second_set.clone()
  ));
  println!("second isn't a proper subset of the first one? {}", is_not_proper_subset(
      first_set.clone(),
      second_set.clone()
  ));
  sets_vector.push(
    union(
      first_set.clone(),
      second_set.clone()
));

println!("{:?}", sets_vector);

sets_vector.push(
    intersection(
      first_set.clone(),
      second_set.clone()
));

println!("{:?}", sets_vector);

let mut cartesian_products: Vec<Vec<(i32, i32)>> = Vec::new();

cartesian_products.push(
  cartesian_product(
    first_set.clone(),
    second_set.clone()
));

println!("{:?}", cartesian_products);

println!("{:?}", revert_cartesian_product(cartesian_products.into_iter().nth(0).expect("no set on this position")));

sets_vector.push(
  subtraction(
    first_set.clone(),
    second_set.clone()
  )
);

println!("{:?}", sets_vector);

let mut power_set_vec: Vec<Vec<IndexSet<i32>>> = Vec::new();

power_set_vec.push(power_set(first_set.clone()));

println!("{:?}", power_set_vec.clone().into_iter().nth(0).expect("no power set on this position"));


println!("{:?}", revert_power_set(
  power_set_vec.clone().into_iter().nth(0).expect("no power set on this position")
));

}

fn main() {
  let sets_vector = read_sets_from_file();
    testing(sets_vector);
}
