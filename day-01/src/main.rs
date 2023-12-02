fn main() {
    // Example input string
    let input_string = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";

    // Call the function with the input string
    if let Some(sum) = calculate_sum(input_string) {
        // Print the sum of all calibration values
        println!("Sum of calibration values: {}", sum);
    } else {
        // Handle the case where parsing fails
        println!("Error: Unable to parse calibration values.");
    }
}

fn calculate_sum(input: &str) -> Option<u32> {
    // Split the input into lines
    let lines: Vec<&str> = input.lines().collect();

    // Initialize a variable to store the sum of calibration values
    let mut sum = 0;

    // Iterate over each line in the input
    for line in lines {
        // Extract the first and last characters of the line
        let first_char = line.chars().next()?;
        let last_char = line.chars().rev().next()?;

        // Convert the characters to digits and form a two-digit number
        let calibration_value = format!("{}{}", first_char, last_char)
            .parse::<u32>()
            .ok()?;

        // Add the calibration value to the sum
        sum += calibration_value;
    }

    // Return the sum
    Some(sum)
}
