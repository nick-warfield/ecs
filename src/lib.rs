#![feature(bool_to_option)]

// just for testing
#[derive(Debug, Default, PartialEq)]
pub struct Name(String);
#[derive(Debug, Default, PartialEq)]
pub struct Position(f32, f32);
#[derive(Debug, Default, PartialEq)]
pub struct Velocity(f32, f32);

#[derive(Debug, Copy, Clone)]
pub struct Entity {
	generation: usize,
	index: usize,
}

const ALIVE:	u16 = 0b0000_0000_0000_0001;
const NAME:		u16 = 0b0000_0000_0000_0010;
const POSITION:	u16 = 0b0000_0000_0000_0100;
const VELOCITY:	u16 = 0b0000_0000_0000_1000;

pub struct ComponentList<'a> {
	pub name: Option<&'a Name>,
	pub positon: Option<&'a Position>,
	pub velocity: Option<&'a Velocity>,
}

#[derive(Debug)]
pub struct System {
	next_allocation: Vec<usize>,
	generations: Vec<usize>,

	components: Vec<u16>,
	name: Vec<Name>,
	pos: Vec<Position>,
	vel: Vec<Velocity>,
}

impl System {
	pub fn new() -> System {
		System {
			next_allocation: vec![],
			generations: vec![],
			components: vec![],
			name: vec![],
			pos: vec![],
			vel: vec![],
		}
	}

	pub fn create_entity(&mut self, components: u16) -> Entity {
		let components = components | ALIVE;
		if let Some(next) = self.next_allocation.pop() {
			self.generations[next] += 1;
			self.components[next] = components;

			Entity {
				index: next,
				generation: self.generations[next],
			}
		} else {
			self.generations.push(0);
			self.components.push(components);
			Entity {
				index: self.generations.len() - 1,
				generation: 0,
			}
		}
	}

	pub fn remove(&mut self, ent: &Entity) {
		if self.is_alive(ent) {
			self.components[ent.index] &= !ALIVE;
			self.next_allocation.push(ent.index);
		}
	}

	pub fn get(&self, ent: &Entity) -> Option<ComponentList> {
		self.is_alive(ent).then_some(ComponentList {
			name: self.has_components(ent, NAME).then_some(&self.name[ent.index]),
			positon: self.has_components(ent, POSITION).then_some(&self.pos[ent.index]),
			velocity: self.has_components(ent, VELOCITY).then_some(&self.vel[ent.index]),
		})
	}

	pub fn get_mut(&mut self, ent: &Entity) -> Option<ComponentList> {
		self.is_alive(ent).then_some(ComponentList {
			name: self.has_components(ent, NAME).then_some(&self.name[ent.index]),
			positon: self.has_components(ent, POSITION).then_some(&self.pos[ent.index]),
			velocity: self.has_components(ent, VELOCITY).then_some(&self.vel[ent.index]),
		})
	}

	pub fn is_alive(&self, ent: &Entity) -> bool {
		self.generations[ent.index] == ent.generation
			&& self.components[ent.index] & ALIVE == ALIVE
	}

	fn has_components(&self, ent: &Entity, components: u16) -> bool {
		self.components[ent.index] & components == components
	}
}

#[cfg(test)]
pub mod tests {
	use crate::*;

	#[test]
	fn create_entities() {
		let mut ecs = System::new();
		let e1 = ecs.create_entity(
			Name(format!("Pilot Pete")),
			Position(5.0, 5.0),
			Velocity(8.0, 0.0),
			);
		let e2 = ecs.create_entity(
			Name(format!("Tame Impala")),
			Position(0.1, -50.0),
			Velocity(0.0, -10.0),
			);

		assert_eq!(0, e1.index);
		assert_eq!(0, e1.generation);
		assert_eq!(
			Name(format!("Pilot Pete")),
			*ecs.get::<Name>(&e1).unwrap());
		assert_eq!(
			Position(5.0, 5.0),
			*ecs.get::<Position>(&e1).unwrap());
		assert_eq!(
			Velocity(8.0, 0.0),
			*ecs.get::<Velocity>(&e1).unwrap());

		assert_eq!(1, e2.index);
		assert_eq!(0, e2.generation);
		assert_eq!(
			Name(format!("Tame Impala")),
			*ecs.get::<Name>(&e2).unwrap());
		assert_eq!(
			Position(0.1, -50.0),
			*ecs.get::<Position>(&e2).unwrap());
		assert_eq!(
			Velocity(0.0, -10.0),
			*ecs.get::<Velocity>(&e2).unwrap());
	}

	#[test]
	fn remove_entity()
	{
		let mut ecs = System::new();
		let e1 = ecs.create_entity(
			Name(format!("Pilot Pete")),
			Position(5.0, 5.0),
			Velocity(8.0, 0.0),
			);

		ecs.remove(&e1);
		assert_eq!(None, ecs.get::<Name>(&e1));
		ecs.remove(&e1);
		assert_eq!(None, ecs.get::<Name>(&e1));
	}

	#[test]
	fn stack_allocation()
	{
		let mut ecs = System::new();
		let e1 = ecs.create_entity(
			Name(format!("Pilot Pete")),
			Position(5.0, 5.0),
			Velocity(8.0, 0.0),
			);
		let e2 = ecs.create_entity(
			Name(format!("Tame Impala")),
			Position(0.1, -50.0),
			Velocity(0.0, -10.0),
			);
		
		ecs.remove(&e1);
		let e1 = ecs.create_entity(
			Name(format!("Hannah Montana")),
			Position(-0.1, -5.0),
			Velocity(0.0, -11.0),
			);
		let e3 = ecs.create_entity(
			Name(format!("Pilot Pete")),
			Position(5.0, 5.0),
			Velocity(8.0, 0.0),
			);

		assert_eq!(0, e1.index);
		assert_eq!(2, e1.generation);
		assert_eq!(
			Name(format!("Hannah Montana")),
			*ecs.get::<Name>(&e1).unwrap());
		assert_eq!(
			Position(-0.1, -5.0),
			*ecs.get::<Position>(&e1).unwrap());
		assert_eq!(
			Velocity(0.0, -11.0),
			*ecs.get::<Velocity>(&e1).unwrap());

		assert_eq!(1, e2.index);
		assert_eq!(0, e2.generation);
		assert_eq!(
			Name(format!("Tame Impala")),
			*ecs.get::<Name>(&e2).unwrap());
		assert_eq!(
			Position(0.1, -50.0),
			*ecs.get::<Position>(&e2).unwrap());
		assert_eq!(
			Velocity(0.0, -10.0),
			*ecs.get::<Velocity>(&e2).unwrap());

		assert_eq!(2, e3.index);
		assert_eq!(0, e3.generation);
		assert_eq!(
			Name(format!("Pilot Pete")),
			*ecs.get::<Name>(&e3).unwrap());
		assert_eq!(
			Position(5.0, 5.0),
			*ecs.get::<Position>(&e3).unwrap());
		assert_eq!(
			Velocity(8.0, 0.0),
			*ecs.get::<Velocity>(&e3).unwrap());
	}

	#[test]
	fn different_generations()
	{
		let mut ecs = System::new();
		let e1 = ecs.create_entity(
			Name(format!("Pilot Pete")),
			Position(5.0, 5.0),
			Velocity(8.0, 0.0),
			);
		ecs.remove(&e1);

		let e2 = ecs.create_entity(
			Name(format!("Tame Impala")),
			Position(0.1, -50.0),
			Velocity(0.0, -10.0),
			);

		assert_eq!(0, e1.index);
		assert_eq!(0, e1.generation);
		assert_eq!(None, ecs.get::<Name>(&e1));

		assert_eq!(0, e2.index);
		assert_eq!(2, e2.generation);
		assert_eq!(
			Name(format!("Tame Impala")),
			*ecs.get::<Name>(&e2).unwrap());
		assert_eq!(
			Position(0.1, -50.0),
			*ecs.get::<Position>(&e2).unwrap());
		assert_eq!(
			Velocity(0.0, -10.0),
			*ecs.get::<Velocity>(&e2).unwrap());
	}

	#[test]
	#[should_panic]
	fn access_undeclared_component_panics()
	{
		let mut ecs = System::new();
		ecs.push(0);
	}
}
