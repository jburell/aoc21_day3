pub fn calc_power_consumption(data: Vec<u32>, num_bits: usize) -> u32 {
  let r: Vec<u32> = 
    data.iter()
    .fold(vec![0; num_bits], |acc, bits|{
      acc.into_iter().enumerate()
      .map(|(n, v)| {
        v + ((bits >> n) & 0b1)
      }).collect()
    });

  let num_words_div_2  = data.len() as u32 >> 1;
  let gamma: u32 = 
    r.iter().enumerate()
    .map(|(n, v)| {
      if *v > num_words_div_2 {
        1 << n
      } else {
        0
      }
    })
    .sum();
  let filter: u32 = (0..(num_bits)).into_iter().map(|v| 1 << v).sum();
  println!("{:b}", filter);
  let epsilon = (!gamma) & filter;
  gamma * epsilon
}

pub fn calc_oxygen_and_co2(data: Vec<u32>, num_bits: usize) -> u32 {
  let r: Vec<u32> = 
    data.iter()
    .fold(vec![((0, Vec::new()), 0, Vec::new()); num_bits], |((count_ones, acc_ones), (count_zeroes, acc_zero)), bits|{
      acc.into_iter().enumerate()
      .map(|(n, v)| {
        v + ((bits >> n) & 0b1)
      }).collect()
    });

  let num_words_div_2  = data.len() as u32 >> 1;
  let oxy: u32 = 
    r.iter().enumerate()
    .map(|(n, v)| {
      if *v > num_words_div_2 {
        1 << n
      } else {
        0
      }
    })
    .sum();
  let filter: u32 = (0..(num_bits)).into_iter().map(|v| 1 << v).sum();
  println!("{:b}", filter);
  let epsilon = (!oxy) & filter;
  oxy * epsilon
}
