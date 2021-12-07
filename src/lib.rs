use core::num;
use std::collections::VecDeque;

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

#[derive(Debug, Clone)]
struct Elem((u32, Vec<u32>), (u32, Vec<u32>));
impl Elem {
  pub fn new() -> Self {
    Elem((0, Vec::new()), (0, Vec::new()))
  }
}

pub fn calc_oxygen_and_co2(data: Vec<u32>, num_bits: usize) -> u32 {
  let queues = vec![VecDeque::new(); num_bits];

  Some((queues, data))
  .map(|(q, d)|{
    q.iter().enumerate()
    
  });


  /*let r = 
    data.iter()
    .fold(vec![Elem::new(); num_bits], |acc, bits|{
      let res = 
      acc
      .iter()
      .enumerate()
      .map(|(n, elem)| {
        let (count_zeroes, acc_zeroes) = elem.0;
        let (count_ones, acc_ones) = elem.1;
        match (bits >> n) & 0b1 {
          0 => {
            acc_zeroes.push(*bits);
            Elem((count_zeroes + 1, acc_zeroes), (count_ones, acc_ones))
          }
          1 => {
            acc_ones.push(*bits);
            Elem((count_zeroes, acc_zeroes), (count_ones + 1, acc_ones))
          }
          _ => panic!("only ones and zeroes possible")

        }
      })
      .collect();

      res
    })
    .fold(Vec::new(), |acc, vec| {
      acc.push(vec);
      acc
    });*/

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
