// Copyright 2020 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

//! Tests for benchmarking macro.

	// populate_set {
	// 	let x in 0 .. 10_000;
	// 	let mut m = Vec::<u32>::new();
	// 	for i in 0..x {
	// 		m.push(i);
	// 	}
	// }: {
	// 	let _ = m.into_iter().collect::<BTreeSet<u32>>();
	// 	Ok(()) 
	// }