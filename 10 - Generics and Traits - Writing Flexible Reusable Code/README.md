# Generics and Traits

Generics are a bit tricky, therefore the project inside this folder will be as simple and as to the point as possible.

## The Basics of Generics

```
fn solve<T: Float>(a: T, b: T) -> T {
	(a.powi(2) + b.powi(2)).sqrt()
}
```
**Generic Type**: Like an argument list, but for types

A generic type allows us to provide some customiation or really flexibility around not only functions, but methods, enums, vectors, structs.
<br/>
Just about everything in Rust can work with generics, and the goal of these generic types are to make your code a little bit more flexible and work with different types.

The angle brackets `<T: Float>` is a generic list.
- You can think of this as being like an argument list, but specifically for types.
- `Float` is referred to as a `Trait`. In this context, at this location where it's being used, it is referred to as a `trait bound`.

### What's a Trait ?

- A trait is a set of methods
- It can contain **abstract methods** which don't have an implementation
- It can contain **default methods** which have an implementation 

```
trait Vehicle {
	// abstract method
	fn start(&self);

	// default method
	fn stop(&self) {
		println!("Stopped");
	}
}
```
In this example, the name of the trait is *Vehicle*. It has an abstract method `start()`, and a default method `stop()`.

Once we have created a trait, we can then choose to implement it with different types.
- A struct/enum/primitive can **implement** a trait.
- The implementor has to provide an implementation for **all the abstract methods**
- The implementor can **optionally** override the default methods

```
trait Vehicle {
	// abstract method
	fn start(&self);

	// default method
	fn stop(&self) {
		println!("Stopped");
	}
}

struct Car {};

impl Vehicle for Car {
	fn start(&self) {
		println!("Start!!!");
	}
}
```
In this example, since we have implemented a trait `Vehicle`, that means that this `Car` gets two big benefits.
- First, the `Car` is considered to be of type `Vehicle` 
- Second, it gets access to all the different methods that were defined inside the trait

### How would we use this in practice ?

```
fn start_and_stop<T: Vehicle>(vehicule: T) {
	vehicle.start();

	vehicle.stop();
}

fn main() {
	let car = Car {};

	start_and_stop(car);
}
```
In this example, 
- The way that this function has been set up is a generic function
- `T` must be something that implements the `Vehicle` trait, so we can pass in any kind of vehicle so long as it implements the `Vehicle` trait
- `Car` has implemented that trait so we can safely pass it into `start_and_stop()`
- Inside that function, we get access to the `vehicule` argument, and we can call all the methods tied to that trait.

So in the `<T: Vehicle>`, `Vehicle` itself is a **trait**, and in this context, is serving as a **trait bound**.
<br/>
*It is saying that whatever argument you pass in this type T, it must implement the vehicle trait.*

Now back to our original method (cf Section_10-01_Project_Generics): 
```
fn solve_generic<T: Float>(a: T, b: T) -> f64 {
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();

    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

fn main() {
    let a: f32 = 3.0;
    let b: f32 = 4.0;

    println!("{}", solve_generic::<f32>(a, b));
}
We can pass as A and B both a `f32` or a `f64`, as both `f32` and `f64` implement the `Float` trait.
}
```
