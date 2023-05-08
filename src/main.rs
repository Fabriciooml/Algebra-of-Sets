use indexmap::set::IndexSet;
use std::fs::File;
use std::io::{BufRead, BufReader, stdin};

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
          println!("{:?}", subset);
          result.push(subset);
      }
  }
  result
}

fn revert_power_set(power_set_vec: Vec<IndexSet<i32>>) -> IndexSet<i32> {
  let mut reverted: IndexSet<i32> = IndexSet::new();
  for i in 0..power_set_vec.len() {
    reverted = union(reverted, power_set_vec.clone().into_iter().nth(i).expect("no set on this position"));
  }
  reverted
}

fn get_relation_matrix(set_a: IndexSet<i32>, set_b: IndexSet<i32>, operation:&str, inputed_relation:Vec<(i32, i32)>) -> Vec<Vec<bool>> {
  // A coluna é o set A as linhas são o set B
  let mut relation = vec![vec![false; set_a.len()]; set_b.len()];
  for i in 0..set_a.len() {
    for j in 0..set_b.len() {
      match operation {
        "smaller" => {
          let value_a = set_a.clone().into_iter().nth(i).expect("");
          let value_b = set_b.clone().into_iter().nth(j).expect("");
          if value_a < value_b {
            relation[j][i] = true;
          }
        }
        "bigger" => {
          let value_a = set_a.clone().into_iter().nth(i).expect("");
          let value_b = set_b.clone().into_iter().nth(j).expect("");
          if value_a > value_b {
            relation[j][i] = true;
          }
        }
        "equals" => {
          let value_a = set_a.clone().into_iter().nth(i).expect("");
          let value_b = set_b.clone().into_iter().nth(j).expect("");
          if value_a == value_b {
            relation[j][i] = true;
          }
        }
        "exponential" => {
          let value_a = set_a.clone().into_iter().nth(i).expect("");
          let value_b = set_b.clone().into_iter().nth(j).expect("");
          if value_a*value_a == value_b {
            relation[j][i] = true;
          }
        }
        "sqrt" => {
          let value_a = set_a.clone().into_iter().nth(i).expect("");
          let value_b = set_b.clone().into_iter().nth(j).expect("");
          if (value_a as f64).sqrt() == (value_b as f64) {
            relation[j][i] = true;
          }
        }
        "relation" => {
          let value_a = set_a.clone().into_iter().nth(i).expect("");
          let value_b = set_b.clone().into_iter().nth(j).expect("");
          for k in 0..inputed_relation.len(){
            let item = inputed_relation.clone().into_iter().nth(k).expect("");
            if value_a == item.0 && value_b == item.1 {
              relation[j][i] = true;
            }
          }
        }
        _ => println!("Invalid Operator")
      }
    }
  }
  relation
}

fn is_functional(relation_matrix: Vec<Vec<bool>>) -> bool {
  for i in 0..relation_matrix.len() {
    let mut counter = 0;
    let row = relation_matrix.clone().into_iter().nth(i).expect("invalid row");
    for j in 0..row.len() {
      let value = row.clone().into_iter().nth(j).expect("Value doesn't exists");
      if value {
        counter += 1;
      }
    }
    if counter > 1 {
      return false;
    }
  }
  true
}

fn is_injector(relation_matrix: Vec<Vec<bool>>) -> bool {
  for i in 0..relation_matrix.len() {
    let mut counter = 0;
    let row = relation_matrix.clone().into_iter().nth(i).expect("invalid row");
    for j in 0..row.len() {
      let value = row.clone().into_iter().nth(j).expect("Value doesn't exists");
      if value {
        counter += 1;
        if counter > 1 {
            return false;
        }
      }
    }
  }
true
}

fn is_total(relation_matrix: Vec<Vec<bool>>) -> bool {
  for i in 0..relation_matrix.len() {
    let mut counter = 0;
    let row = relation_matrix.clone().into_iter().nth(i).expect("invalid row");
    for j in 0..row.len() {
      let value = row.clone().into_iter().nth(j).expect("Value doesn't exists");
      if value {
        counter += 1;
      }
    }
    if counter < 1 {
      return false;
    }
  }
  true
}

fn is_surjective(relation_matrix: Vec<Vec<bool>>) -> bool {
  for i in 0..relation_matrix.len() {
    let mut counter = 0;
    let row = relation_matrix.clone().into_iter().nth(i).expect("invalid row");
    for j in 0..row.len() {
      let value = row.clone().into_iter().nth(j).expect("Value doesn't exists");
      if value {
        counter += 1;
        if counter < 1 {
            return false;
        }
      }
    }
  }
true
}

fn is_monomorphism(relation_matrix: Vec<Vec<bool>>) -> bool {
  if is_total(relation_matrix.clone()) && is_injector(relation_matrix.clone()) {
    return true;
  }
  false
}

fn is_epimorphism(relation_matrix: Vec<Vec<bool>>) -> bool {
  if is_functional(relation_matrix.clone()) && is_surjective(relation_matrix.clone()) {
    return true;
  }
  false
}

fn is_isomorphism(relation_matrix: Vec<Vec<bool>>) -> bool {
  if is_monomorphism(relation_matrix.clone()) && is_epimorphism(relation_matrix.clone()) {
    return true;
  }
  false
}

fn identify_relation(relation_matrix: Vec<Vec<bool>>) -> Vec<String> {
  let mut result:Vec<String> = Vec::new();
  if is_functional(relation_matrix.clone()) {
    result.push(String::from("Functional"));
  }
  if is_injector(relation_matrix.clone()) {
    result.push(String::from("Injector"));
  }
  if is_total(relation_matrix.clone()) {
    result.push(String::from("Total"));
  }
  if is_surjective(relation_matrix.clone()) {
    result.push(String::from("Surjective"));
  }
  if is_monomorphism(relation_matrix.clone()) {
    result.push(String::from("Monomorphism"));
  }
  if is_epimorphism(relation_matrix.clone()) {
    result.push(String::from("Epimorphism"));
  }
  if is_isomorphism(relation_matrix.clone()) {
    result.push(String::from("Isomorphism"));
  }
  result
}

fn get_user_input() -> String {
  let mut user_input:String = String::new();
  stdin().read_line(&mut user_input).expect("Did not enter a correct string");
  if let Some('\n')=user_input.chars().next_back() {
    user_input.pop();
  }
  if let Some('\r')=user_input.chars().next_back() {
    user_input.pop();
  }
  user_input
}

fn tui(mut sets_vector: Vec<IndexSet<i32>>) {
  let mut user_input = String::new();
  while user_input != "q"{
    println!("{}", "-".repeat(50));
    println!("Algebra of Sets");
    println!("{}", "-".repeat(50));
    println!("Menu:");
    println!("0  - Belongs");
    println!("1  - Does not belongs");
    println!("2  - Subset");
    println!("3  - Not a subset");
    println!("4  - Proper subset");
    println!("5  - Not proper subset");
    println!("6  - Union");
    println!("7  - Intersect");
    println!("8  - Subtract");
    println!("9  - Cartesian product");
    println!("10 - Power set");
    println!("r - Relations");
    println!("p  - Print sets vector");
    println!("q  - Quit");

    user_input = get_user_input();

    println!("{}", "-".repeat(50));
    
    if user_input == "0" { //Belongs
      println!("Choose a set:");
      for (i, item) in sets_vector.iter().enumerate() {
          println!("{} = {:?}", i, item);
      }

      let choosen_set_index: usize = get_user_input().trim().parse().expect("Input not an integer");
      let choosen_set = sets_vector.clone().into_iter().nth(choosen_set_index).expect("Set not found");
      println!("{}", "-".repeat(50));

      println!("Insert a number to see if it belongs to the set:");
      let choosen_number_str = get_user_input();
      let choosen_number_int: i32 = choosen_number_str.trim().parse().expect("Input not an integer");
      println!("{}", "-".repeat(50));

      match belongs(choosen_set, &choosen_number_int) {
        true => println!("\n{} belongs to the set\n", choosen_number_int),
        false => println!("\n{} doesn't belong to the set\n", choosen_number_int)
      }
    }
  
    else if user_input == "1" { //Does not belongs
      println!("Choose a set:");
      for (i, item) in sets_vector.iter().enumerate() {
          println!("{} = {:?}", i, item);
      }

      let choosen_set_index: usize = get_user_input().trim().parse().expect("Input not an integer");
      let choosen_set = sets_vector.clone().into_iter().nth(choosen_set_index).expect("Set not found");
      println!("{}", "-".repeat(50));

      println!("Insert a number to see if it belongs to the set:");
      let choosen_number_str = get_user_input();
      let choosen_number_int: i32 = choosen_number_str.trim().parse().expect("Input not an integer");
      println!("{}", "-".repeat(50));

      match does_not_belong(choosen_set, &choosen_number_int) {
        true => println!("\n{} doesn't belong to the set\n", choosen_number_int),
        false => println!("\n{} belongs to the set\n", choosen_number_int)
      }
    }
  
    else if user_input == "2" { //Subset

      println!("Choose the first set:");
      for (i, item) in sets_vector.iter().enumerate() {
          println!("{} = {:?}", i, item);
      }

      let choosen_first_set_index: usize = get_user_input().trim().parse().expect("Input not an integer");
      let choosen_first_set = sets_vector.clone().into_iter().nth(choosen_first_set_index).expect("Set not found");
      println!("{}", "-".repeat(50));

      println!("Choose the second set:");
      for (i, item) in sets_vector.iter().enumerate() {
          println!("{} = {:?}", i, item);
      }

      let choosen_second_set_index: usize = get_user_input().trim().parse().expect("Input not an integer");
      let choosen_second_set = sets_vector.clone().into_iter().nth(choosen_second_set_index).expect("Set not found");
      println!("{}", "-".repeat(50));

      match is_subset(choosen_first_set, choosen_second_set) {
        true => println!("\nSecond set is a subset of the first one\n"),
        false => println!("\nSecond set isn't a subset of the first one\n")
      }
    }

    else if user_input == "3" { //Not a subset

      println!("Choose the first set:");
      for (i, item) in sets_vector.iter().enumerate() {
          println!("{} = {:?}", i, item);
      }

      let choosen_first_set_index: usize = get_user_input().trim().parse().expect("Input not an integer");
      let choosen_first_set = sets_vector.clone().into_iter().nth(choosen_first_set_index).expect("Set not found");
      println!("{}", "-".repeat(50));

      println!("Choose the second set:");
      for (i, item) in sets_vector.iter().enumerate() {
          println!("{} = {:?}", i, item);
      }

      let choosen_second_set_index: usize = get_user_input().trim().parse().expect("Input not an integer");
      let choosen_second_set = sets_vector.clone().into_iter().nth(choosen_second_set_index).expect("Set not found");
      println!("{}", "-".repeat(50));

      match is_not_subset(choosen_first_set, choosen_second_set) {
        true => println!("\nSecond set isn't a subset of the first one\n"),
        false => println!("\nSecond set is a subset of the first one\n")
      }
    }
  
    else if user_input == "4" { //Proper subset

      println!("Choose the first set:");
      for (i, item) in sets_vector.iter().enumerate() {
          println!("{} = {:?}", i, item);
      }

      let choosen_first_set_index: usize = get_user_input().trim().parse().expect("Input not an integer");
      let choosen_first_set = sets_vector.clone().into_iter().nth(choosen_first_set_index).expect("Set not found");
      println!("{}", "-".repeat(50));

      println!("Choose the second set:");
      for (i, item) in sets_vector.iter().enumerate() {
          println!("{} = {:?}", i, item);
      }

      let choosen_second_set_index: usize = get_user_input().trim().parse().expect("Input not an integer");
      let choosen_second_set = sets_vector.clone().into_iter().nth(choosen_second_set_index).expect("Set not found");
      println!("{}", "-".repeat(50));

      match is_proper_subset(choosen_first_set, choosen_second_set) {
        true => println!("\nSecond set is a proper subset of the first one\n"),
        false => println!("\nSecond set isn't a proper subset of the first one\n")
      }
    }

    else if user_input == "5" { //Not proper subset

      println!("Choose the first set:");
      for (i, item) in sets_vector.iter().enumerate() {
          println!("{} = {:?}", i, item);
      }

      let choosen_first_set_index: usize = get_user_input().trim().parse().expect("Input not an integer");
      let choosen_first_set = sets_vector.clone().into_iter().nth(choosen_first_set_index).expect("Set not found");
      println!("{}", "-".repeat(50));

      println!("Choose the second set:");
      for (i, item) in sets_vector.iter().enumerate() {
          println!("{} = {:?}", i, item);
      }

      let choosen_second_set_index: usize = get_user_input().trim().parse().expect("Input not an integer");
      let choosen_second_set = sets_vector.clone().into_iter().nth(choosen_second_set_index).expect("Set not found");
      println!("{}", "-".repeat(50));

      match is_not_proper_subset(choosen_first_set, choosen_second_set) {
        true => println!("\nSecond set isn't a proper subset of the first one\n"),
        false => println!("\nSecond set is a proper subset of the first one\n")
      }
    }
  
    else if user_input == "6" { //Union

      println!("Choose a set:");
      for (i, item) in sets_vector.iter().enumerate() {
          println!("{} = {:?}", i, item);
      }

      println!("{}", "-".repeat(50));

      user_input = get_user_input();

      println!("{user_input}");

      let choosen_first_set_index: usize = user_input.trim().parse().expect("Input not an integer");
      let choosen_first_set = sets_vector.clone().into_iter().nth(choosen_first_set_index).expect("Set not found");
      println!("{}", "-".repeat(50));

      println!("Choose the second set:");
      for (i, item) in sets_vector.iter().enumerate() {
          println!("{} = {:?}", i, item);
      }

      let choosen_second_set_index: usize = get_user_input().trim().parse().expect("Input not an integer");
      let choosen_second_set = sets_vector.clone().into_iter().nth(choosen_second_set_index).expect("Set not found");
      
      let unioned = union(choosen_first_set, choosen_second_set);
      
      println!("{}", "-".repeat(50));
      
      println!("\n{:?}\n", unioned);

      sets_vector.push(unioned);
      println!("Added unioned set to sets vector");
      println!("\n{:?}\n", sets_vector);
    }

    else if user_input == "7" { //Intersection
      println!("Choose a set:");
      for (i, item) in sets_vector.iter().enumerate() {
          println!("{} = {:?}", i, item);
      }

      println!("{}", "-".repeat(50));

      user_input = get_user_input();

      println!("{user_input}");

      let choosen_first_set_index: usize = user_input.trim().parse().expect("Input not an integer");
      let choosen_first_set = sets_vector.clone().into_iter().nth(choosen_first_set_index).expect("Set not found");
      println!("{}", "-".repeat(50));

      println!("Choose the second set:");
      for (i, item) in sets_vector.iter().enumerate() {
          println!("{} = {:?}", i, item);
      }

      let choosen_second_set_index: usize = get_user_input().trim().parse().expect("Input not an integer");
      let choosen_second_set = sets_vector.clone().into_iter().nth(choosen_second_set_index).expect("Set not found");
      
      let intersected = intersection(choosen_first_set, choosen_second_set);
      
      println!("{}", "-".repeat(50));
    
      println!("\n{:?}\n", intersected);

      sets_vector.push(intersected);
      println!("Added intersected set to sets vector");
      println!("\n{:?}\n", sets_vector);
    }

    else if user_input == "8" { //Subtraction 
      println!("Choose the first set:");
      for (i, item) in sets_vector.iter().enumerate() {
          println!("{} = {:?}", i, item);
      }

      let choosen_first_set_index: usize = get_user_input().trim().parse().expect("Input not an integer");
      let choosen_first_set = sets_vector.clone().into_iter().nth(choosen_first_set_index).expect("Set not found");
      println!("{}", "-".repeat(50));

      println!("Choose the second set:");
      for (i, item) in sets_vector.iter().enumerate() {
          println!("{} = {:?}", i, item);
      }

      let choosen_second_set_index: usize = get_user_input().trim().parse().expect("Input not an integer");
      let choosen_second_set = sets_vector.clone().into_iter().nth(choosen_second_set_index).expect("Set not found");
      println!("{}", "-".repeat(50));

      let subtracted = subtraction(choosen_first_set, choosen_second_set);

      println!("\n{:?}\n", subtracted);

      sets_vector.push(subtracted);
      println!("Added subtracted set to sets vector");
      println!("\n{:?}\n", sets_vector);
    }
  
    else if user_input == "9" { //Cartesian Product 
      println!("Choose the first set:");
      for (i, item) in sets_vector.iter().enumerate() {
          println!("{} = {:?}", i, item);
      }

      let choosen_first_set_index: usize = get_user_input().trim().parse().expect("Input not an integer");
      let choosen_first_set = sets_vector.clone().into_iter().nth(choosen_first_set_index).expect("Set not found");
      println!("{}", "-".repeat(50));

      println!("Choose the second set:");
      for (i, item) in sets_vector.iter().enumerate() {
          println!("{} = {:?}", i, item);
      }

      let choosen_second_set_index: usize = get_user_input().trim().parse().expect("Input not an integer");
      let choosen_second_set = sets_vector.clone().into_iter().nth(choosen_second_set_index).expect("Set not found");
      println!("{}", "-".repeat(50));

      let product = cartesian_product(choosen_first_set, choosen_second_set);

      println!("\n{:?}\n", product);

      println!("Want to revert it? y/n");
      match get_user_input().trim(){
        "y" => {println!("{:?}", revert_cartesian_product(product))},
        "n" => {continue},
        _   => {println!("Invalid")}
      }

    }

    else if user_input == "10" { //Power set
      println!("Choose the first set:");
      for (i, item) in sets_vector.iter().enumerate() {
          println!("{} = {:?}", i, item);
      }

      let choosen_set_index: usize = get_user_input().trim().parse().expect("Input not an integer");
      let choosen_set = sets_vector.clone().into_iter().nth(choosen_set_index).expect("Set not found");
      println!("{}", "-".repeat(50));

      let result_set = power_set(choosen_set);

      println!("\n{:?}\n", result_set);

      println!("Want to revert it? y/n");
      match get_user_input().trim(){
        "y" => {println!("{:?}", revert_power_set(result_set))},
        "n" => {continue},
        _   => {println!("Invalid")}
      }
    }
  
    else if user_input == "r" { //Relations menu
      let mut relations_user_input = String::new();
      while relations_user_input != "q"{

        println!("{}", "-".repeat(50));
        println!("Algebra of Sets - Relations");
        println!("{}", "-".repeat(50));
        println!("Menu:");
        println!(" 0 - Smaller then (<)");
        println!(" 1 - Bigger then (>)");
        println!(" 2 - Equals to (=)");
        println!(" 3 - Exponential (**)");
        println!(" 4 - Square root (sqrt)");
        println!(" 5 - User input relation");
        println!(" q - Back to menu");


        relations_user_input = get_user_input();
        
        if relations_user_input == "q" {
          break;
        }
        let mut inputed_relation:Vec<(i32, i32)> = Vec::new();

        println!("Choose the first set:");
        for (i, item) in sets_vector.iter().enumerate() {
            println!("{} = {:?}", i, item);
        }
  
        let choosen_first_set_index: usize = get_user_input().trim().parse().expect("Input not an integer");
        let choosen_first_set = sets_vector.clone().into_iter().nth(choosen_first_set_index).expect("Set not found");
        println!("{}", "-".repeat(50));
  
        println!("Choose the second set:");
        for (i, item) in sets_vector.iter().enumerate() {
            println!("{} = {:?}", i, item);
        }
  
        let choosen_second_set_index: usize = get_user_input().trim().parse().expect("Input not an integer");
        let choosen_second_set = sets_vector.clone().into_iter().nth(choosen_second_set_index).expect("Set not found");
        println!("{}", "-".repeat(50));

        let mut operation = "";

        if relations_user_input == "0" {
          operation = "smaller";
        }

        else if relations_user_input == "1" {
          operation = "bigger";
        }

        else if relations_user_input == "2" {
          operation = "equals";
        }
 
        else if relations_user_input == "3" {
          operation = "exponential";
        }

        else if relations_user_input == "4" {
          operation = "sqrt";
        }

        else if relations_user_input == "5" {
          operation = "relation";
          println!("{}", "-".repeat(50));
          println!("Input the size of the relations vector: ");
          let mut size:i32 = get_user_input().trim().parse().expect("Input not an integer");
          while size != 0{
            println!("input first element: ");
            let first_element = get_user_input().trim().parse().expect("Input not an integer");
            println!("input second element: ");
            let second_element = get_user_input().trim().parse().expect("Input not an integer");
            inputed_relation.push((first_element, second_element));
            size -= 1;
        }
      }
      
      let relation_matrix = get_relation_matrix(choosen_first_set, choosen_second_set, operation, inputed_relation);
      let relation_classification = identify_relation(relation_matrix);

      println!("The relation is:");
      for classification in relation_classification {
        println!("{:?}", classification);
      }
    }
  }

    else if user_input == "p" { // Print sets
      for (i, item) in sets_vector.iter().enumerate() {
        println!("{} = {:?}", i, item);
      }
    }
  }
}

fn main() {
  let sets_vector = read_sets_from_file();
  tui(sets_vector);
}
