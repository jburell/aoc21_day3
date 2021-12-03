use std::fs;
use aoc21_day3::*;

fn get_data(filename: &str) -> (Vec<u32>, usize) {
  let data_str: Vec<String> = 
    fs::read_to_string(filename).unwrap()
    .lines()
    .map(|s| s.trim().to_owned())
    .collect();
  let num_bits = data_str[0].len();
  let data =
    data_str.iter()
    .filter(|s| !s.is_empty())
    .map(|value|u32::from_str_radix(&value,2).unwrap())
    .collect();
  (data, num_bits)
}

#[test]
fn part_1() {
  // Arrange
  let data = get_data("input_part1.txt");

  // Act
  let actual = calc_power_consumption(data.0, data.1);
  // Assert
  assert_eq!(2954600, actual);
}
