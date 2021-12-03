pub fn calc_power_consumption(data: Vec<u32>, num_bits: usize) -> u32 {
  let r: Vec<u32> = 
    data.iter()
    .fold(vec![0; num_bits], |acc, pentad|{
      acc.into_iter().enumerate()
      .map(|(n, v)| {
        v + ((pentad >> n) & 0b1)
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