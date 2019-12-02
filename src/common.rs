
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct InitialCell {
	pub x: usize,
	pub y: usize,
	pub value: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FractalResult {
	pub initial_configuration: Vec<InitialCell>,
	pub sand_data: Vec<u8>,
	pub count_data: Vec<u32>,
	pub side_length: usize,

	pub total_redistributions: i64,
	pub total_iterations: usize,
}