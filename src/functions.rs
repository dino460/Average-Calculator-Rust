use colored::*;
pub use std::io;


pub fn check_if_empty(values_vec: &mut Vec<f32>, sample_size: usize)
{
    if values_vec.len() != sample_size {
        change_vec_values(values_vec, sample_size);
    }
}


// Value calculation functions
pub fn average(values_vec: &Vec<f32>, sample_size: usize) -> f32 {
    let mut average_value: f32 = 0.0;

    for value in values_vec {
        average_value += value;
    }

    return average_value / (sample_size as f32);
}

pub fn weighed_average(values_vec: &Vec<f32>, weights_vec: &Vec<f32>) -> f32 {
    let mut weighed_average_value: f32 = 0.0;
    let mut weight_sum: f32 = 0.0;

    for (index, value) in values_vec.iter().enumerate() {
        weighed_average_value += value * weights_vec[index];
        weight_sum += weights_vec[index];
    }

    return weighed_average_value / weight_sum;
}

pub fn standard_deviation(values_vec: &Vec<f32>, sample_size: usize) -> f32 {
    let mut standard_deviation_value: f32 = 0.0;
    let average = average(values_vec, sample_size);

    for value in values_vec {
        let x = value - average;
        standard_deviation_value += x.powf(2.0);
    }
    
    standard_deviation_value = standard_deviation_value / (sample_size as f32);

    return standard_deviation_value.sqrt();
}

pub fn get_biggest_value(values_vec: &Vec<f32>) -> f32 {
    let mut biggest_value: f32 = values_vec[0];

    for value in values_vec {
        if *value > biggest_value {
            biggest_value = *value;
        }
    }

    return biggest_value;
}

pub fn get_smallest_value(values_vec: &Vec<f32>) -> f32 {
    let mut smallest_value: f32 = values_vec[0];

    for value in values_vec {
        if *value < smallest_value {
            smallest_value = *value;
        }
    }

    return smallest_value;
}


// Vector manipulation functions
pub fn change_vec_values(vec: &mut Vec<f32>, sample_size: usize) {
    vec.clear();

    let mut input: String = String::new();

    match io::stdin().read_line(&mut input) {
        Err(error) => {
            println!("{} {error}", "(!) Invalid input. Exiting method.\nerror:".bright_red().bold());
            return;
        },
        _ => (),
    };
    input.pop();

    let values_vec: Vec<&str> = input.split(' ').collect();

    let array_size = if values_vec.len() < sample_size { 
        values_vec.len() 
    } else {
        sample_size
    };

    for i in 0..array_size {
        let input_as_f32: f32 = match values_vec[i].parse::<f32>() {
            Ok(v) => v,
            Err(_) => 0.0
        };

        vec.push(input_as_f32);
    }

    for _ in values_vec.len()..sample_size {
        vec.push(0.0);
    }

    //vec
}

pub fn reset_vec_values(vec: &mut Vec<f32>, sample_size: usize) {
    vec.clear();

    for _ in 0..sample_size {
        vec.push(1.0);
    }
}

pub fn change_vec_size(values_vec: &mut Vec<f32>, sample_size: usize) {
    values_vec.clear();
    values_vec.resize(sample_size, 0.0);
}

pub fn change_vec_value_at_index(vec: &mut Vec<f32>)
{
    let mut input: String = String::new();

    println!("\n{}", "Type the index to insert into:".italic());

    match io::stdin().read_line(&mut input) {
        Err(error) => {
            println!("{} {error}", "(!) Invalid input. Exiting method.\nerror:".bright_red().bold());
            return;
        },
        _ => (),
    };
    input.pop();

    let input_as_usize: usize = match input.parse::<usize>() {
        Ok(v) => v,
        Err(error) => {
            println!("{} {error}", "(!) Invalid input. Exiting method.\nerror:".bright_red().bold());
            return;
        }
    };

    if input_as_usize >= vec.len() {
        println!("{}", "(!) Index out of bounds".bright_red().bold());
        return;
    }

    input.clear();


    println!("\n{}", "Type the value to into:".italic());

    match io::stdin().read_line(&mut input) {
        Err(error) => {
            println!("{} {error}", "(!) Invalid input. Exiting method.\nerror:".bright_red().bold());
            return;
        },
        _ => (),
    };
    input.pop();

    let input_as_f32: f32 = match input.parse::<f32>() {
        Ok(v) => v,
        Err(error) => {
            println!("{} {error}", "(!) Invalid input. Exiting method.\nerror:".bright_red().bold());
            return;
        }
    };

    vec[input_as_usize] = input_as_f32;
}
