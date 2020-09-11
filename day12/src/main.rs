//155 is too low
//3413 is too low

#[derive(Debug)]
struct Moon {
	x: isize,
	y: isize,
	z: isize,
	vx: isize,
	vy: isize,
	vz: isize,
}

impl Moon {
	fn new(x: isize, y: isize, z: isize) -> Self {
		Self {
			x,
			y,
			z,
			vx: 0,
			vy: 0,
			vz: 0,
		}
	}

	fn pot(&self) -> isize {
		self.x.abs() + self.y.abs() + self.z.abs()
	}

	fn kin(&self) -> isize {
		self.vx.abs() + self.vy.abs() + self.vz.abs()
	}
}

// fn print_moon(m: Moon) {println!("{:?}", m);}
// fn print_moon_ref(m: &Moon) {println!("{:?}", m);}

#[derive(Debug)]
struct System {
	moons: Vec<Moon>,
}

impl System {
	fn new(moons: Vec<Moon>) -> Self {
		Self {
			moons,
		}
	}

	/// Update the velocity and position of each moon in the system
	fn tick(&mut self) {
		// Calcualte delta vs
		let mut delta_vs = Vec::<(isize, isize, isize)>::new();
		for m1 in &self.moons {
			let mut dvx = 0;
			let mut dvy = 0;
			let mut dvz = 0;

			for m2 in &self.moons {
				if m1.x < m2.x {
					dvx += 1
				}
				else if m1.x > m2.x {
					dvx -= 1
				}

				if m1.y < m2.y {
					dvy += 1
				}
				else if m1.y > m2.y {
					dvy -= 1
				}

				if m1.z < m2.z {
					dvz += 1
				}
				else if m1.z > m2.z {
					dvz -= 1
				}
			}

			delta_vs.push((dvx, dvy, dvz))
		}

		// Print the delta vs
		// println!("{:?}", delta_vs);


		// Update the velocities

		// println!("Moons before velocity update: {:?}", self.moons);

		// for (&mut moon, dv) in self.moons.iter_mut().zip(delta_vs.iter()) {
		// 	moon.vx += dv.0;
		// 	moon.vy += dv.1;
		// 	moon.vz += dv.2;
		// }

		// todo yikes, hardcoded moon list length
		for i in 0..4 {
			self.moons[i].vx += delta_vs[i].0;
			self.moons[i].vy += delta_vs[i].1;
			self.moons[i].vz += delta_vs[i].2;
		}

		// println!("Moons after  velocity update: {:?}", self.moons);

		// Update positions
		for mut moon in &mut self.moons {
			moon.x += moon.vx;
			moon.y += moon.vy;
			moon.z += moon.vz;
		}

		// println!("Moons after  position update: {:?}", self.moons);
	}

	/// Calculate the total energy of the system
	fn energy(&self) -> isize {
		//TODO use a fold here?
		let mut e = 0;
		for moon in &self.moons {
			e += moon.pot() * moon.kin();
		}
		e
	}
}

fn main() {
	//TODO actually parse the input
	let moons = vec![
		Moon::new(-1, 7, 3),
		Moon::new(12, 2, -13),
		Moon::new(14, 18, -8),
		Moon::new(17, 4, -4),
	];

	// Sample input 1
	// let moons = vec![
	// 	Moon::new(-1, 0, 2),
	// 	Moon::new(2, -10, -7),
	// 	Moon::new(4, -8, 8),
	// 	Moon::new(3, 5, -1),
	// ];

	// Sample input 2
	// let moons = vec![
	// 	Moon::new(-8, -10, 0),
	// 	Moon::new(5, 5, 10),
	// 	Moon::new(2, -7, 3),
	// 	Moon::new(9, -8, -3),
	// ];

	let mut system = System::new(moons);
	println!("System before simulation: {:?}", system);
	println!("Total energy {:?}", system.energy());

	println!("Starting conditions");

	// Simulate 1000 time steps
	for step in 0..1000 {
		system.tick();

		println!("\nstep {}", step);
		println!("System: {:?}", system);
		println!("Total energy {:?}", system.energy());
	}


}
