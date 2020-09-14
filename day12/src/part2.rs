use std::hash::Hash;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::Hasher;
use num_integer::Integer;


#[derive(Debug, Hash, PartialEq, Eq)]
struct Particle {
	position: isize,
	velocity: isize,
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct LinearSystem {
	particles: Vec<Particle>,
}

impl LinearSystem {
	/// Create a new instance given only positions, velocities ar assumed to be zero
	fn new(positions: Vec<isize>) -> Self {
		Self {
			particles: positions.iter().map(|p| Particle{position: *p, velocity: 0}).collect(),
		}
	}

	/// Update the velocity and position of each particle in the system
	fn tick(&mut self) {
		// Calcualte delta vs
		let mut delta_vs = Vec::<isize>::new();
		for p1 in &self.particles {
			let mut dv = 0;

			for p2 in &self.particles {
				if p1.position < p2.position {
					dv += 1
				}
				else if p1.position > p2.position {
					dv -= 1
				}
			}

			delta_vs.push(dv);
		}

		// println!("The delta vs are {:?}", delta_vs);

		// Update the velocities
		for (particle, delta_v) in self.particles.iter_mut().zip(delta_vs.iter()) {
			particle.velocity += delta_v;
		}

		// println!("Particles after  velocity update: {:?}", self.particles);

		// Update positions
		for mut particle in &mut self.particles {
			particle.position += particle.velocity;
		}

		// println!("Particles after position update: {:?}", self.particles);
	}

	fn steps_to_repeat(&mut self) -> usize {
		let mut visited = HashSet::<u64>::new();
		let mut steps = 0;

		let mut hasher = DefaultHasher::new();
		self.hash(&mut hasher);
		let mut state_hash = hasher.finish();

		// Similate the repetition of the x system
		while !visited.contains(&state_hash) || steps == 0 {
			visited.insert(state_hash);
			steps += 1;
			self.tick();

			let mut hasher = DefaultHasher::new();
			self.hash(&mut hasher);
			state_hash = hasher.finish();
		}

		steps
	}
}

pub fn solve() {
	//TODO actually parse the input
	let mut x_system = LinearSystem::new(vec![-1, 12, 14, 17]);
	let mut y_system = LinearSystem::new(vec![7, 2, 18, 4]);
	let mut z_system = LinearSystem::new(vec![3, -13, -8, -4]);

	// Sample input 1
	// let mut x_system = LinearSystem::new(vec![-1, 2, 4, 3]);
	// let mut y_system = LinearSystem::new(vec![0, -10, -8, 5]);
	// let mut z_system = LinearSystem::new(vec![2, -7, 8, -1]);

	// Sample input 2 (Not used in part 2)
	// let mut x_system = LinearSystem::new(vec![-8, 5, 2, 9]);
	// let mut y_system = LinearSystem::new(vec![-10, 5, -7, -8]);
	// let mut z_system = LinearSystem::new(vec![0, 3, -8, -3]);

	// Sample input 3 (Only used in part 2)
	// let mut x_system = LinearSystem::new(vec![-8, 5, 2, 9]);
	// let mut y_system = LinearSystem::new(vec![-10, 5, -7, -8]);
	// let mut z_system = LinearSystem::new(vec![0, 10, 3, -3]);

	// Simulate each dimension
	let x_steps = x_system.steps_to_repeat();
	let y_steps = y_system.steps_to_repeat();
	let z_steps = z_system.steps_to_repeat();
	// println!("Steps before the x axis repeats {}", x_steps);
	// println!("Steps before the y axis repeats {}", y_steps);
	// println!("Steps before the z axis repeats {}", z_steps);

	// The entire system repeats after the LCM of each dimensions repeat period
	let total_repeat = x_steps.lcm(&y_steps.lcm(&z_steps));
	println!("Total repeat period {}", total_repeat);
}
