use anymap::AnyMap;

// just for testing
#[derive(Debug, Default, PartialEq)]
pub struct Name(String);
#[derive(Debug, Default, PartialEq)]
pub struct Position(f32, f32);
#[derive(Debug, Default, PartialEq)]
pub struct Velocity(f32, f32);

type NameData = Vec::<Option::<Name>>;
type PositionData = Vec::<Option::<Position>>;
type VelocityData = Vec::<Option::<Velocity>>;

#[derive(Debug, Copy, Clone)]
pub struct Entity
{
	generation: usize,
	index: usize,
}

#[derive(Debug)]
pub struct System
{
	next_allocation: Vec<usize>,
	generations: Vec<usize>,		// odd == dead
	data: AnyMap,
}

impl System
{
	pub fn new() -> System
	{
		let mut data = AnyMap::new();
		data.insert::<NameData>(vec![]);
		data.insert::<PositionData>(vec![]);
		data.insert::<VelocityData>(vec![]);

		System {
			next_allocation: vec![],
			generations: vec![],
			data,
		}
	}

	#[inline]
	fn push<T: 'static>(&mut self, value: T)
	{
		self.data.get_mut::<Vec<Option<T>>>()
			.expect(format!(
				"Type: {} was not added to data",
				std::any::type_name::<T>()).as_str())
			.push(Some(value));
	}

	#[inline]
	fn insert<T: 'static>(&mut self, value: T, index: usize)
	{
		self.data.get_mut::<Vec<Option<T>>>()
			.expect(format!(
				"Type: {} was not added to data",
				std::any::type_name::<T>()).as_str())
			[index] = Some(value);
	}


	pub fn create_entity(
		&mut self,
		name: Name,
		pos: Position,
		vel: Velocity,
		) -> Entity
	{
		if let Some(next) = self.next_allocation.pop()
		{
			self.generations[next] += 1;
			self.insert(name, next);
			self.insert(pos, next);
			self.insert(vel, next);

			Entity {
				index: next,
				generation: self.generations[next],
			}
		}
		else
		{
			self.generations.push(0);
			self.push(name);
			self.push(pos);
			self.push(vel);
			Entity {
				index: self.generations.len() - 1,
				generation: 0,
			}
		}
	}

	pub fn remove(&mut self, ent: &Entity) -> Option<()>
	{
		if let Some(gen) = self.generations.get(ent.index)
		{
			if *gen % 2 == 0
			{
				self.generations[ent.index] += 1;
				self.next_allocation.push(ent.index);
				None
			}
			else { None }
		}
		else { None }
	}

	pub fn get<T: 'static>(&self, ent: &Entity) -> Option<&T>
	{
		if let Some(gen) = self.generations.get(ent.index)
		{
			if *gen == ent.generation
			{
				self.data.get::<Vec<Option<T>>>()
					.unwrap()[ent.index]
					.as_ref()
			}
			else { None }
		}
		else { None }
	}

	pub fn get_mut<T: 'static>(&mut self, ent: &Entity) -> Option<&mut T>
	{
		if let Some(gen) = self.generations.get(ent.index)
		{
			if *gen == ent.generation
			{
				self.data.get_mut::<Vec<Option<T>>>()
					.expect(format!(
						"Type: {} was not added to data",
						std::any::type_name::<T>()).as_str())
					[ent.index]
					.as_mut()
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
