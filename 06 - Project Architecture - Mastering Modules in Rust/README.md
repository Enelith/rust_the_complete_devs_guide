# Modules


## Modules Overview
The Media Project is mostly complete, but there's a problem with it: our code is not clean.

```
// Project

// main.rs

enum Media {}

struct Catalog {}

fn main() {}

```

- **main.rs is getting messy**
- We could probably organize our code a little better by using **modules**
- Modules group together related code (could be Enums, Functions, Structs, or anything else)

Our goal here will be to move the `enum Media` and the `struct Catalog` (along with their implementations) into a submodule called `content`

```
// Project

// root module: 
fn main() {}

// content module:
enum Media {}

struct Catalog {}
```

Doing this refactor is going to end up being a little confusing, because it turns out there are several different ways.

### Three ways we can make modules

#### Option 1: Create a mod in an existing file
```
mod content {
	pub enum Media { /* fields */ }
	/* Media impl */

	pub struct Catalog { /* fields */ }
	/* Catalog impl */
}

fn main() {
	let catalog = content::Catalog::new();
}
``` 
- Most appropriate when you have a really large file with a lot of stuff going on (but you still want to keep those things inside the same file).

**Note:** `pub` keyword. Functions, structs, enums, etc must have the `pub` keyword to make them visible outside the module.

---

#### Option 2: Create a module in a new single file in the same folder
***src/content.rs***
```
// src/content.rs
pub enum Media { /* fields */ }
/* Media impl */

pub struct Catalog { /* fields */ }
/* Catalog impl */
```


***src/main.rs***
```
mod content; // name of the file that contains our module.

fn main() {
	let catalog = content::Catalog::new();
}
``` 
- Most appropriate when you want a separate module to organize code, but it doesn't need to span several files.

---

#### Option 3: Spread code out among several separae files in a new folder

***'content' folder***
```
// content/media.rs
pub enum Media { /* fields */ }
/* Media impl */

// content/catalog.rs
pub struct Catalog { /* fields */ }
/* Catalog impl */

// content/mod.rs
pub mod media;
pub mod catalog;
```


***src/main.rs***
```
mod content; 

fn main() {
	let catalog = content::catalog::Catalog::new();
}
``` 

- Most appropriate when you have a **large module**
- **Has a couple of confusing parts**


