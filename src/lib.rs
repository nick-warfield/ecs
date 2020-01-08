// just for testing
#[derive(Debug, PartialEq)]
pub struct Name<'a>(&'a str);
#[derive(Debug, PartialEq)]
pub struct Position(f32, f32);
#[derive(Debug, PartialEq)]
pub struct Velocity(f32, f32);

// this is just a generational index?
#[derive(Debug)]
pub struct Entity
{
	pub index: usize,
}


// name this better, it's accurate but long
// should implement a Struct of Arrays Pattern
#[derive(Default, Debug)]
pub struct EntityComponentSystem<'a>
{
	names: Vec<Name<'a>>,
	positions: Vec<Position>,
	velocities: Vec<Velocity>,
}
impl<'a> EntityComponentSystem<'a>
{
	pub fn create_entity(
		&mut self,
		name: &'a str, 
		position: Position,
		velocity: Velocity,
		) -> Entity
	{
		self.names.push(Name(name));
		self.positions.push(position);
		self.velocities.push(velocity);
		Entity { index: 0 }
	}
	pub fn get(&self, ent: &Entity)
		-> (&'a Name, &'a Position, &'a Velocity)
	{
		(&self.names[ent.index],
		 &self.positions[ent.index],
		 &self.velocities[ent.index],
		 )
	}
}

#[cfg(test)]
pub mod tests
{
	use crate::*;

	#[test]
	fn create_entity()
	{
		let mut ecs: EntityComponentSystem = Default::default();
		let e = ecs.create_entity(
			"Pilot Pete",
			Position(5.0, 5.0),
			Velocity(8.0, 0.0),
			);

		let (name, pos, vel) = ecs.get(&e);
		assert_eq!(Name("Pilot Pete"), *name);
		assert_eq!(Position(5.0, 5.0), *pos);
		assert_eq!(Velocity(8.0, 0.0), *vel);
	}
}
