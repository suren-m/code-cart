trait LandCapable {
    fn drive(&self) -> String {
        String::from("driving")
    }
}

struct Sedan;
impl LandCapable for Sedan {
    fn drive(&self) -> String {
        String::from("driving a sedan")
    }
}
struct SUV;
impl LandCapable for SUV {
    fn drive(&self) -> String {
        String::from("driving a SUV")
    }
}
trait WaterCapable {
    fn float(&self) -> String;
}

/// Super Trait
trait Amphibious: LandCapable + WaterCapable {}

struct HoverCraft;
impl Amphibious for HoverCraft {}
impl LandCapable for HoverCraft {}
impl WaterCapable for HoverCraft {
    fn float(&self) -> String {
        String::from("hovercraft on water")
    }
}

/// Generic function with a where constraint
/// Equivalent to fn road_trip<T: LandCapable>(vehicle: &T) -> String {}
/// Constraint must be a trait
/// T: Anything that implements LandCapable
fn road_trip<T>(vehicle: &T) -> String
where
    T: LandCapable,
{
    vehicle.drive()
}

/// Static Dispatch with `impl` parameter
fn road_trip_impl(vehicle: &impl LandCapable) -> String {
    vehicle.drive()
}

/// Dynamic Dispatch with `dyn` parameter
/// Added cost with fat Pointers and vtables
fn road_trip_dyn(vehicle: &dyn LandCapable) -> String {
    vehicle.drive()
}

fn traverse_frozen_lake(vehicle: &impl Amphibious) -> Vec<String> {
    let mut status = Vec::with_capacity(2);
    status.push(vehicle.drive());
    status.push(vehicle.float());

    status
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generic_func_test() {
        let car = Sedan;
        assert_eq!(car.drive(), String::from("driving a sedan"));

        // call via interface
        assert_eq!(LandCapable::drive(&car), String::from("driving a sedan"));

        // call via struct
        assert_eq!(Sedan::drive(&car), String::from("driving a sedan"));

        // generic function call
        assert_eq!(road_trip(&car), String::from("driving a sedan"));
        //above call is same as below. (Similar to doing something like Collect::<String>();)
        assert_eq!(road_trip::<Sedan>(&car), String::from("driving a sedan"));

        let truck = SUV;
        assert_eq!(truck.drive(), String::from("driving a SUV"));
        assert_eq!(road_trip(&truck), String::from("driving a SUV"));
    }

    #[test]
    fn static_dispatch_test() {
        let car = Sedan;
        assert_eq!(car.drive(), String::from("driving a sedan"));

        assert_eq!(road_trip_impl(&car), String::from("driving a sedan"));
        // can't do below as it does not take generic type params
        //assert_eq!(road_trip_impl::<Sedan>(&car), String::from("driving a sedan"));

        let truck = SUV;
        assert_eq!(truck.drive(), String::from("driving a SUV"));
        assert_eq!(road_trip_impl(&truck), String::from("driving a SUV"));
    }

    #[test]
    fn dynamic_dispatch_test() {
        let car = Sedan;
        assert_eq!(car.drive(), String::from("driving a sedan"));
        assert_eq!(road_trip_dyn(&car), String::from("driving a sedan"));

        let truck = SUV;
        assert_eq!(truck.drive(), String::from("driving a SUV"));
        assert_eq!(road_trip_dyn(&truck), String::from("driving a SUV"));
    }

    #[test]
    fn super_trait_test() {
        let hc = HoverCraft;
        let expected = vec![String::from("driving"), String::from("hovercraft on water")];
        assert_eq!(traverse_frozen_lake(&hc), expected);
    }
}
