pub use std::io;


// Value calculation functions
pub fn average(values_vec: &Vec<f32>, sample_size: usize) -> f32 {
    let mut average_value: f32 = 0.0;

    for value in values_vec {
        average_value += value;
    }

    return average_value / (sample_size as f32);
}

pub fn weighed_average(values_vec: &Vec<f32>, weights_vec: &Vec<f32>, sample_size: usize) -> f32 {
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
pub fn change_vec_values(mut vec: Vec<f32>, sample_size: usize) -> Vec<f32> {
    vec.clear();
    vec = Vec::with_capacity(sample_size);

    let mut input: String = String::new();

    for index in 0..sample_size {
        io::stdin().read_line(&mut input);
        input.pop();
    
        let input_as_f32: f32 = match input.parse::<f32>() {
            Ok(v) => v,
            Err(_) => 0.0
        };

        vec.push(input_as_f32);
        input.clear();
    }

    vec
}

pub fn change_vec_size(values_vec: Vec<f32>, mut sample_size: usize) -> Vec<f32> {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input);
    
    let input_as_usize: usize = match input.parse::<usize>() {
        Ok(v) => v,
        Err(_) => 5
    };
    
    sample_size = input_as_usize;

    Vec::with_capacity(input_as_usize)
}
