use std::fmt;
use std::cmp::Ordering;

use serde_json::{Result, Value};

#[derive(Debug, Default)]
pub struct Packet {
    packets: Vec<(Value, Value)>,
    ordered_packets: Vec<Value>,
    temp_value: Value,
    distress_sum: usize,
    decoder_multiplier: usize,
}

impl Packet {
    pub fn new() -> Self {
        Packet {
            decoder_multiplier: 1,
            ..Default::default()
        }
    }

    pub fn parse(&mut self, line: &str) -> Result<()> {
        let value: Value = serde_json::from_str(line)?;

        self.ordered_packets.push(value.clone());

        // println!("Input: {}", value);

        if self.temp_value != Value::Null {
            self.packets.push((self.temp_value.clone(), value));
            self.temp_value = Value::Null;
        } else {
            self.temp_value = value;
            // println!();
        }

        Ok(())
    }

    pub fn compute_indicies(&mut self) -> Result<()> {
        for (idx, (left_v, right_v)) in self.packets.iter().enumerate() {
            let ret = Packet::compare_left_and_right(left_v, right_v);

            // println!("Return value for {} vs {}: {:?}", left_v, right_v, ret);
            // println!();

            if ret.is_lt() {
                self.distress_sum += idx + 1;
            }
        }

        Ok(())
    }

    fn compare_left_and_right(left: &Value, right: &Value) -> Ordering {
        for i in 0..5 {
            let mut left_v = left[i].clone();
            let mut right_v = right[i].clone();

            // println!("Compare {} vs {}", left_v, right_v);

            // Check for null values first
            if left_v.is_null() && !right_v.is_null() {
                return Ordering::Less;
            } else if !left_v.is_null() && right_v.is_null() {
                return Ordering::Greater;
            }
            else if left_v.is_null() && right_v.is_null() {
                return Ordering::Equal;
            }

            // Check to see if both of our values are numbers. If not (which means one of them
            // is an Array), convert the number to an Array and compute from there.
            if left_v.is_number() && right_v.is_array() {
                left_v = Value::Array(vec![left_v]);
            } else if left_v.is_array() && right_v.is_number() {
                right_v = Value::Array(vec![right_v]);
            } else if left_v.is_number() && right_v.is_number() {
                if left_v.as_u64() < right_v.as_u64() {
                    return Ordering::Less;
                } else if left_v.as_u64() > right_v.as_u64() {
                    return Ordering::Greater;
                }
                else if left_v == right_v {
                    continue;
                }
            }

            let ret = Packet::compare_left_and_right(&left_v, &right_v);

            if ret == Ordering::Equal {
                continue;
            } else {
                return ret;
            }
        }

        // Should never actually be reached according to the input, it's just here as a return
        // value so that the compiler doesn't complain.
        Ordering::Equal
    }

    pub fn sort_packets(&mut self) -> Result<()> {
        self.ordered_packets.push(serde_json::from_str("[[2]]")?);
        // println!("[[2]] pushed.");
        self.ordered_packets.push(serde_json::from_str("[[6]]")?);
        // println!("[[6]] pushed.");
        // println!();

        // println!("Sorting packets...");
        self.ordered_packets.sort_by(Packet::compare_left_and_right);
        // println!("Packets sorted successfully.");

        Ok(())
    }

    pub fn compute_sorted_indicies(&mut self) -> Result<()> {
        for (idx, value) in self.ordered_packets.iter().enumerate() {
            let two: Value = serde_json::from_str("[[2]]")?;
            let six: Value = serde_json::from_str("[[6]]")?;

            if *value == two || *value == six {
                self.decoder_multiplier *= idx + 1;
            }
        }

        Ok(())
    }

    pub fn do_part_one(&self) {
        println!("Part One:");
        println!("    The sum of the distress signal indicies is {}", self.distress_sum);
    }

    pub fn do_part_two(&self) {
        println!("Part Two:");
        println!("    The decoder multiplier of indicies is {}", self.decoder_multiplier);
    }
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (left, right) in self.packets.iter() {
            writeln!(f, "  Left:  {:?}", left).expect("Should be able to write left.");
            writeln!(f, "  Right: {:?}", right).expect("Should be able to write right.");
            writeln!(f).expect("Should be able to write newline.");
        }

        fmt::Result::Ok(())
    }
}
