// just for testing
#[derive(Debug, Default, PartialEq)]
pub struct Name<'a>(&'a str);
#[derive(Debug, Default, PartialEq)]
pub struct Position(f32, f32);
#[derive(Debug, Default, PartialEq)]
pub struct Velocity(f32, f32);

#[derive(Debug, Copy, Clone)]
pub struct Entity
{
	generation: usize,
	index: usize,
}

#[derive(Debug, Copy, Clone)]
pub struct Entry<T: Default>
{
	value: T,
	generation: usize,
}
impl<T: Default> Entry<T>
{
	pub fn new(value: T, generation: usize) -> Entry<T>
	{
		Entry {
			value,
			generation,
		}
	}

	pub fn as_ref(&self) -> Option<&T>
	{
		if self.is_alive() { Some(&self.value) }
		else { None }
	}
	pub fn as_mut(&mut self) -> Option<&mut T>
	{
		if self.is_alive() { Some(&mut self.value) }
		else { None }
	}

	pub fn is_alive(&self) -> bool { self.generation == 0 }
	pub fn generation(&self) -> usize { self.generation }
	pub fn kill(&mut self) -> Option<T>
	{
		if self.is_alive()
		{
			self.generation = 0;
			Some(std::mem::take(&mut self.value))
		}
		else
		{
			None
		}
	}
}

// name this better, it's accurate but long
#[derive(Debug)]
pub struct EntityComponentSystem<'a>
{
	next_allocation: Entity,
	names: Vec<Entry<Name<'a>>>,
	positions: Vec<Entry<Position>>,
	velocities: Vec<Entry<Velocity>>,
}

impl<'a> EntityComponentSystem<'a>
{
	pub fn new() -> EntityComponentSystem<'a>
	{
		EntityComponentSystem {
			next_allocation: Entity { index: 0, generation: 1 },
			names: vec![],
			positions: vec![],
			velocities: vec![],
		}
	}
	pub fn create_entity(
		&mut self,
		name: &'a str, 
		position: Position,
		velocity: Velocity,
		) -> Entity
	{
		let next = self.next_allocation.index;
		let gen = self.next_allocation.generation;
		if next == self.names.len()
		{
			self.names.push(Entry::new(Name(name), gen));
			self.positions.push(Entry::new(position, gen));
			self.velocities.push(Entry::new(velocity, gen));
		}
		else
		{
			self.names[next] = Entry::new(Name(name), gen);
			self.positions[next] = Entry::new(position, gen);
			self.velocities[next] = Entry::new(velocity, gen);
		}

		let entity = self.next_allocation;
		self.next_allocation.index += 1;
		entity
	}

	pub fn remove_entity(&mut self, ent: &Entity)
	{
		self.names[ent.index].kill();
		self.positions[ent.index].kill();
		self.velocities[ent.index].kill();
		self.next_allocation.index = ent.index;
	}

	pub fn get(&self, ent: &Entity)
		-> Option<(&'a Name, &'a Position, &'a Velocity)>
	{
		if !self.names[ent.index].is_alive()
			|| ent.index != self.next_allocation.index
		{ 
			None
		}
		else if let (Some(name), Some(pos), Some(vel)) = (
			self.names[ent.index].as_ref(),
			self.positions[ent.index].as_ref(),
			self.velocities[ent.index].as_ref(),
			)
		{
			Some((name, pos, vel))
		}
		else
		{
			panic!("Unhandled Case Detected");
		}
	}
}

#[cfg(test)]
pub mod tests
{
	use crate::*;

	#[test]
	fn create_entities()
	{
		let mut ecs = EntityComponentSystem::new();
		let e1 = ecs.create_entity(
			"Pilot Pete",
			Position(5.0, 5.0),
			Velocity(8.0, 0.0),
			);
		let e2 = ecs.create_entity(
			"Tame Impala",
			Position(0.1, -50.0),
			Velocity(0.0, -10.0),
			);

		assert_eq!(1, e1.generation);
		assert_eq!(0, e1.index);

		// this kind of test is broken
		if let Some((name, pos, vel)) = ecs.get(&e1)
		{
			assert_eq!(Name("Pilot Pete"), *name);
			assert_eq!(Position(5.0, 5.0), *pos);
			assert_eq!(Velocity(8.0, 0.0), *vel);
		}

		assert_eq!(1, e2.generation);
		assert_eq!(1, e2.index);
		if let Some((name, pos, vel)) = ecs.get(&e2)
		{
			assert_eq!(Name("Tame Impala"), *name);
			assert_eq!(Position(0.1, -50.0), *pos);
			assert_eq!(Velocity(0.0, -10.0), *vel);
		}
	}

	#[test]
	fn remove_entity()
	{
		let mut ecs = EntityComponentSystem::new();
		let e1 = ecs.create_entity(
			"Pilot Pete",
			Position(5.0, 5.0),
			Velocity(8.0, 0.0),
			);

		ecs.remove_entity(&e1);
		assert_eq!(None, ecs.get(&e1));
		ecs.remove_entity(&e1);
		assert_eq!(None, ecs.get(&e1));
	}

	#[test]
	fn overwrite_removed_entity()
	{
		let mut ecs = EntityComponentSystem::new();
		let e1 = ecs.create_entity(
			"Pilot Pete",
			Position(5.0, 5.0),
			Velocity(8.0, 0.0),
			);
		let e2 = ecs.create_entity(
			"Tame Impala",
			Position(0.1, -50.0),
			Velocity(0.0, -10.0),
			);
		
		ecs.remove_entity(&e1);
		assert_eq!(None, ecs.get(&e1));

		assert_eq!(e2.index, 1);
		assert_eq!(e2.generation, 1);
		if let Some((name, pos, vel)) = ecs.get(&e2)
		{
			assert_eq!(Name("Tame Impala"), *name);
			assert_eq!(Position(0.1, -50.0), *pos);
			assert_eq!(Velocity(0.0, -10.0), *vel);
		}

		let e1 = ecs.create_entity(
			"Hannah Montana",
			Position(-0.1, -5.0),
			Velocity(0.0, -11.0),
			);
		let e3 = ecs.create_entity(
			"Pilot Pete",
			Position(5.0, 5.0),
			Velocity(8.0, 0.0),
			);

		assert_eq!(e1.index, 0);
		assert_eq!(e1.generation, 1);
		if let Some((name, pos, vel)) = ecs.get(&e1)
		{
			assert_eq!(Name("Hannah Montana"), *name);
			assert_eq!(Position(-0.1, -5.0), *pos);
			assert_eq!(Velocity(0.0, -11.0), *vel);
		}

		assert_eq!(e2.index, 1);
		assert_eq!(e2.generation, 1);
		if let Some((name, pos, vel)) = ecs.get(&e2)
		{
			assert_eq!(Name("Tame Impala"), *name);
			assert_eq!(Position(0.1, -50.0), *pos);
			assert_eq!(Velocity(0.0, -10.0), *vel);
		}

		assert_eq!(e3.index, 2);
		assert_eq!(e3.generation, 1);
		if let Some((name, pos, vel)) = ecs.get(&e3)
		{
			assert_eq!(Name("Pilot Pete"), *name);
			assert_eq!(Position(5.0, 5.0), *pos);
			assert_eq!(Velocity(8.0, 0.0), *vel);
		}
	}
}
