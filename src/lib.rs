// #![feature(bool_to_option)]
#[macro_use]
extern crate casey;

lower!(
#[derive(Debug, Copy, Clone)]
pub struct Entity {
	generation: usize,
	index: usize,
});

// pub trait System {
// 	fn new() -> Self;
// 	fn create(&mut self) -> Entity;
// 	fn delete(&mut self, entity: &Entity);
// }

#[macro_export]
macro_rules! make_world {
	($component:ident) => {
		make_system!({$component, $crate::casey::snake!($component)})
	}
}

#[macro_export]
macro_rules! make_system {
	( $({ $component:ident, $c_field:ident }),* ) => {
		#[derive(Debug)]
		pub struct World {
			next_allocation: Vec<usize>,
			generations: Vec<usize>,
			component_bit_field: Vec<u16>,

			$(pub $c_field: Vec<$component>,)*
		}

		$( $crate::casey::upper!($c_field) = 0; ),*

		impl World {
			pub fn new() -> World {
				World {
					next_allocation: vec![],
					generations: vec![],
					component_bit_field: vec![],

					$($c_field: vec![],)*
				}
			}
		}

	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[derive(Debug)]
	pub struct Position;
	#[derive(Debug)]
	pub struct Velocity;

	#[test]
	fn dummy() {
		//make_system!( {Position, position}, {Velocity, velocity} );
		make_system!({Position, position});
		let w = World::new();

		//panic!("{:?}", w);
	}
}

// just for testing
// #[derive(Debug, Default, PartialEq)]
// pub struct Name(String);
// #[derive(Debug, Default, PartialEq)]
// pub struct Position(f32, f32);
// #[derive(Debug, Default, PartialEq)]
// pub struct Velocity(f32, f32);

// const ALIVE:	u16 = 0b0000_0000_0000_0001;
// const NAME:		u16 = 0b0000_0000_0000_0010;
// const POSITION:	u16 = 0b0000_0000_0000_0100;
// const VELOCITY:	u16 = 0b0000_0000_0000_1000;

// #[derive(Debug)]
// pub enum Component<'a> {
// 	Name(&'a Name),
// 	Position(&'a Position),
// 	Velocity(&'a Velocity),
// }

// impl Entity {
// 	pub fn is_alive(&self, world: &System) -> bool {
// 		world.generations[self.index] == self.generation
// 			&& world.components[self.index] & ALIVE == ALIVE
// 	}
// 
// 	pub fn has_components(&self, world: &System, components: u16) -> bool {
// 		world.components[self.index] & components == components
// 	}
// 
//	pub fn get(&self, world: &'static System, components: u16) -> Result<Vec<Component>, &'static str> {
//		if !self.is_alive(world) {
//			Err("Entity is not alive")
//		} else if !self.has_components(world, components) {
//			Err("Entity does not have all requested components")
//		} else {
//			let mut comps: Vec<Component> = vec![];
//			if components & NAME == NAME {
//				comps.push(Component::Name(&world.name[self.index]));
//			}
//			if components & POSITION == POSITION {
//				comps.push(Component::Position(&world.pos[self.index]));
//			}
//			if components & VELOCITY == VELOCITY {
//				comps.push(Component::Velocity(&world.vel[self.index]));
//			}
//			Ok(comps)
//		}
//	}
// }



// #[derive(Debug)]
// pub struct System {
// 	next_allocation: Vec<usize>,
// 	generations: Vec<usize>,
// 	components: Vec<u16>,
// 
// 	name: Vec<Name>,
// 	pos: Vec<Position>,
// 	vel: Vec<Velocity>,
// }
// 
// impl System {
// 	pub fn new() -> System {
// 		System {
// 			next_allocation: vec![],
// 			generations: vec![],
// 			components: vec![],
// 			name: vec![],
// 			pos: vec![],
// 			vel: vec![],
// 		}
// 	}
// 
// 	pub fn create_entity(&mut self, components: u16) -> Entity {
// 		let components = components | ALIVE;
// 		if let Some(next) = self.next_allocation.pop() {
// 			self.generations[next] += 1;
// 			self.components[next] = components;
// 
// 			Entity {
// 				index: next,
// 				generation: self.generations[next],
// 			}
// 		} else {
// 			self.generations.push(0);
// 			self.components.push(components);
// 			Entity {
// 				index: self.generations.len() - 1,
// 				generation: 0,
// 			}
// 		}
// 	}
// 
// 	pub fn remove_entity(&mut self, ent: &Entity) {
// 		if ent.is_alive(self) {
// 			self.components[ent.index] &= !ALIVE;
// 			self.next_allocation.push(ent.index);
// 		}
// 	}
// }
