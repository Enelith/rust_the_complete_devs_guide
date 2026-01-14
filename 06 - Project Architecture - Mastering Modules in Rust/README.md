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

### Rules of Modules (Option 3)
```
src/
├── main.rs           	<-- Root Module (imports 'content')
└── content/          	<-- Content Module
 ├── mod.rs       		<-- Makes 'content' a module
 ├── media.rs     		<-- Media Module
 └── catalog.rs   		<-- Catalog Module
```

- Every file **and folder** makes its own separate module.
- Anytime we make a **folder** (ex: 'content' folder), we are **required** to put inside of it a file named ***mod.rs***.
<br/>
This folder and this ***mod.rs*** file kind of merge together so to speak, to make another additional module (this module gets named after its folder).

Therefore, based on the previous structure in Option 3, we now got: 
	- root module (because of the file *'src/main.rs'*)
	- content module (because of the folder named *'content'*, and the file *'content/mod.rs'*)
		- media module (because of the file *'content/media.rs'*)
		- catalog module (because of the file *'content/catalog.rs'*)

- You can't do deeply nested imports.
	- For example, at some point of time, we're probably going to want to import the `enum Media` into our *root* module (in another word, the `src/media.rs` file). However, we are **not** allowed to do deeply nested imports.
	- In other words, the *root* module can't reach directly into the *media* module, and get access to something which is defined inside it.
- In order to do so, we'd have to **chain imports**:
	- the *content* module: Being the immediate parent, it imports + exports everything from the *media* module (using `pub mod media`)
	- the *root* module: Can now reach safely into *content*, imports everything from it (using `mod content`), and access to the *media* module
- Whenever we're talking about *importing something*, we're always important stuff on a **one level deep** at a time. 
<br/>
We can't *skip* levels.


