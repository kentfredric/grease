/*!

# `grease-util`

## About

`grease-util` is designed as a low-level multi-command that aims to:

1. Be independent of local user configuration auto-magic, instead, preferring
  to use configuration

2. Implement only as much of PMS as is necessary to use each command correctly

3. Be simple enough to be both useful and composable in simple shell commands

4. Be fast and use minimal memory

## `iterate`

The iterate command is for iterating "kinds" of things in a given repository

```bash
  grease-util iterate --repository PATH_TO_REPO <subcommand>
```

### Subcommands

* `categories`
* `packages`
* `ebuilds`

### Common subcommand arguments

#### `--formatter FORMATTER_NAME`

* Default: `ident`

* Possible: `path`, `ident`, `components`, `name`

##### `path`

Emits the full path to the thing in question:

* `categories`: `/usr/portage/category_name`
* `packages`: `/usr/portage/category_name/package_name`
* `ebuilds`: `/usr/portage/category_name/package_name/ebuild_name.ebuild`

##### `ident`

Emits an "identity" of a thing in question

* `categories`: `category_name/`
* `packages`: `category_name/package_name`
* `ebuilds`: `category_name/package_name-pvr`

##### `name`

Emits a simplified name of the item

* `categories`: `category_name`
* `packages`: `package_name`
* `ebuilds`: `ebuild_name.ebuild`

##### `components`

Emits the thing in question as a space-delimited key=value string

* `categories`: `cat=category_name`
* `packages`: `cat=category_name package=package_name`
* `ebuilds`: `cat=category_name package=package_name version=pvr`

!*/

use std::alloc::System;

#[path = "grease-util/app/mod.rs"]
mod app;

#[global_allocator]
static GLOBAL: System = System;

fn main() { app::run(&app::app().get_matches()) }
