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

// name this better, it's accurate but long
#[derive(Debug)]
pub struct EntityComponentSystem<'a>
{
	next_allocation: Vec<usize>,	// stack of indexes
	lookup: Vec<usize>,				// generations; gen 0 == dead
	names: Vec<Option<Name<'a>>>,
	positions: Vec<Option<Position>>,
	velocities: Vec<Option<Velocity>>,
}

impl<'a> EntityComponentSystem<'a>
{
	pub fn new() -> EntityComponentSystem<'a>
	{
		EntityComponentSystem {
			next_allocation: vec![],
			lookup: vec![],
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
		let index;
		if let Some(next) = self.next_allocation.pop()
		{
			self.lookup[next] = 1;
			self.names[next] = Some(Name(name));
			self.positions[next] = Some(position);
			self.velocities[next] = Some(velocity);
			index = next;
		}
		else
		{
			self.lookup.push(1);
			self.names.push(Some(Name(name)));
			self.positions.push(Some(position));
			self.velocities.push(Some(velocity));
			index = self.lookup.len() - 1;
		}

		Entity { index, generation: 1 }
	}

	pub fn remove_entity(&mut self, ent: &Entity)
	{
		self.lookup[ent.index] = 0;
		self.next_allocation.push(ent.index);
	}

	// make these a macro or template
	pub fn get_name(&self, ent: &Entity) -> Option<&Name>
	{
		if let Some(gen) = self.lookup.get(ent.index)
		{
			if *gen == ent.generation
			{
				self.names[ent.index].as_ref()
			}
			else { None }
		}
		else { None }
	}

	pub fn get_position(&self, ent: &Entity) -> Option<&Position>
	{
		if let Some(gen) = self.lookup.get(ent.index)
		{
			if *gen == ent.generation
			{
				self.positions[ent.index].as_ref()
			}
			else { None }
		}
		else { None }
	}

	pub fn get_velocity(&self, ent: &Entity) -> Option<&Velocity>
	{
		if let Some(gen) = self.lookup.get(ent.index)
		{
			if *gen == ent.generation
			{
				self.velocities[ent.index].as_ref()
			}
			else { None }
		}
		else { None }
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

		assert_eq!(0, e1.index);
		assert_eq!(1, e1.generation);
		assert_eq!(Name("Pilot Pete"), *ecs.get_name(&e1).unwrap());
		assert_eq!(Position(5.0, 5.0), *ecs.get_position(&e1).unwrap());
		assert_eq!(Velocity(8.0, 0.0), *ecs.get_velocity(&e1).unwrap());

		assert_eq!(1, e2.index);
		assert_eq!(1, e2.generation);
		assert_eq!(Name("Tame Impala"), *ecs.get_name(&e2).unwrap());
		assert_eq!(Position(0.1, -50.0), *ecs.get_position(&e2).unwrap());
		assert_eq!(Velocity(0.0, -10.0), *ecs.get_velocity(&e2).unwrap());
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
		assert_eq!(None, ecs.get_name(&e1));
		ecs.remove_entity(&e1);
		assert_eq!(None, ecs.get_name(&e1));
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
		assert_eq!(None, ecs.get_name(&e1));

		assert_eq!(1, e2.index);
		assert_eq!(1, e2.generation);
		assert_eq!(Name("Tame Impala"), *ecs.get_name(&e2).unwrap());
		assert_eq!(Position(0.1, -50.0), *ecs.get_position(&e2).unwrap());
		assert_eq!(Velocity(0.0, -10.0), *ecs.get_velocity(&e2).unwrap());

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

		assert_eq!(0, e1.index);
		assert_eq!(1, e1.generation);
		assert_eq!(Name("Hannah Montana"), *ecs.get_name(&e1).unwrap());
		assert_eq!(Position(-0.1, -5.0), *ecs.get_position(&e1).unwrap());
		assert_eq!(Velocity(0.0, -11.0), *ecs.get_velocity(&e1).unwrap());

		assert_eq!(1, e2.index);
		assert_eq!(1, e2.generation);
		assert_eq!(Name("Tame Impala"), *ecs.get_name(&e2).unwrap());
		assert_eq!(Position(0.1, -50.0), *ecs.get_position(&e2).unwrap());
		assert_eq!(Velocity(0.0, -10.0), *ecs.get_velocity(&e2).unwrap());

		assert_eq!(2, e3.index);
		assert_eq!(1, e3.generation);
		assert_eq!(Name("Pilot Pete"), *ecs.get_name(&e3).unwrap());
		assert_eq!(Position(5.0, 5.0), *ecs.get_position(&e3).unwrap());
		assert_eq!(Velocity(8.0, 0.0), *ecs.get_velocity(&e3).unwrap());

	}
}
