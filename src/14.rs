/* Problem 14: Longest Collatz sequence
 *
 * The following iterative sequence is defined for the set of positive integers:
 *
 * n → n/2 (n is even)
 * n → 3n + 1 (n is odd)
 *
 * Using the rule above and starting with 13, we generate the following sequence:
 * 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
 *
 * It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although
 * it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at
 * 1.
 *
 * Which starting number, under one million, produces the longest chain?
 *
 * NOTE: Once the chain starts the terms are allowed to go above one million. */

extern crate num;
use num::Integer;

static MAX: uint = 1_000_000;

fn main() {
  let master_rx = spawn_workers(MAX);

  let mut result = 0;
  let mut max    = 0;

  for WorkResult { number: num, result: current } in master_rx.iter() {
    if current > max {
      max = current;
      result = num;
    }
  }

  println!("{}", result);
}

fn spawn_workers(max: uint) -> Receiver<WorkResult> {
  use std::os;
  use std::iter::range_step_inclusive;

  let (master_tx, master_rx) = channel();

  let task_count: uint = match os::getenv("NPROC") {
    Some(num) => from_str(num[]).unwrap(),
    None      => 4,
  };

  let per_task = max / task_count;

  for start in range_step_inclusive(1, max, per_task) {
    let master_tx_clone = master_tx.clone();

    spawn(proc() {
      let end = start + per_task;
      collatz_worker((start, end), master_tx_clone);
    });
  }

  return master_rx;
}

struct WorkResult {
  number: u64,
  result: uint,
}

fn collatz_worker(numbers: (uint, uint), tx: Sender<WorkResult>) {
  let (start, end) = numbers;

  let mut current = 0;
  let mut max = 0;

  for num in range(start as u64, end as u64) {
    let len = collatz_length(num);

    if len > max {
      current = num;
      max = len;
    }
  }

  tx.send(WorkResult { number: current, result: max });
}

fn collatz_length(number: u64) -> uint {
  let mut length = 1;
  let mut current_number = number;

  while current_number > 1 {
    current_number =
      if current_number.is_even() {
        current_number / 2
      } else {
        3 * current_number + 1
      };

    length += 1;
  }

  length
}
