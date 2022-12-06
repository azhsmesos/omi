# Omi - Object Mapping Intelligently

A library that maps object structures to relational databases and provides
interfaces for common queries and updates.

[![License](https://img.shields.io/github/license/amphitheatre-app/omi)](https://github.com/amphitheatre-app/omi/blob/master/LICENSE)
[![GitHub contributors](https://img.shields.io/github/contributors/amphitheatre-app/omi)](https://github.com/amphitheatre-app/omi/graphs/contributors)
[![GitHub issues](https://img.shields.io/github/issues/amphitheatre-app/omi)](https://github.com/amphitheatre-app/omi/issues)

## Install

Omi is under active development and when adding a dependency you will need to
add it via the git repository method like the following:

```toml
# Cargo.toml
[dependencies]
omi = { git = "https://github.com/amphitheatre-app/omi" }
```

## How to contribute for omi

### Setting the Environment

#### Installing rustup on Linux or macOS
If you’re using Linux or macOS, open a terminal and enter the following command:
```shell
curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
```
You will also need a linker, which is a program that Rust uses to join its compiled outputs into one file. It is likely you already have one. If you get linker errors, you should install a C compiler, which will typically include a linker. A C compiler is also useful because some common Rust packages depend on C code and will need a C compiler.

On macOS, you can get a C compiler by running:
```shell
 xcode-select --install
```

#### Installing rustup on Windows
On Windows, go to `https://www.rust-lang.org/tools/install` and follow the instructions for installing Rust. At some point in the installation, you’ll receive a message explaining that you’ll also need the MSVC build tools for Visual Studio 2013 or later. To acquire the build tools, you’ll need to install Visual Studio 2022. When asked which workloads to install, include:
- “Desktop Development with C++”
- The Windows 10 or 11 SDK
- The English language pack component, along with any other language pack of your choosing

The rest of this book uses commands that work in both cmd.exe and PowerShell. If there are specific differences, we’ll explain which to use.

#### Set rust-toolchain to nightly-2022-12-01 version
1. When you run the cargo command, you automatically switch to this version
2. Or set the version manually yourself
```shell
rustup install nightly-2022-12-01
rustup override add nightly-2022-12-01
```

## Getting Started

Let's start with a quick preview of some of Omi's main current features, which
will be updated here and in possible future special documents as they become
available, so stay tuned!

## Entity definition

```rust
use omi::{Column, Entity, Relation}

#[Entity(name = "products")]
pub struct Product {
  /// Auto incremented primary key
  #[Column(primary, auto, null)]
  id: u64,

  /// The title of product
  #[Column(length = 255, default = "")]
  title: String

  /// The brand of this product
  #[Relation(belongs_to)]
  brand: Brand,

  /// Has many reviews
  #[Relation(has_many)
  reviews: Vec<Review>,
}
```

## Querying

When the variable is the exact model type, only the first row is obtained, and
the equivalent SQL is `SELECT * FROM products offset 0 limit 1`.

```rust
let product: Product = omi::query::<Product>().execute(db);
```
Fetch all records if the variable is iterable, the equivalent SQL is `SELECT *
FROM products`.

```rust
let products: Vec<Product> = omi::query::<Product>().execute(db);
```

Execute compound queries by specifying the `filter()` method, which takes simple
tuple and tuple vector. It's defined like this: `filter<F>(F)`.
etc.

```rust
// SQL: SELECT * FROM products WHERE id=123
omi::query::<Product>().filter(("id", 123)).execute(db);

// SQL: SELECT * FROM products WHERE foo=123 AND bar=456
omi::query::<Product>()
    .filter([("foo", 123), ("bar", 456)])
    .execute(db);
```

Omi can also be more advanced conditional expressions to support `AND`, `OR`
filtering, the equivalent SQL is `SELECT * FROM products WHERE (title like
"*abc" AND brand_id=123) OR (title like "*xyz" AND brand_id=456)`.

```rust
omi::query::<Product>()
    .filter(
        Or(
            And([("title", "*abc"), ("brand_id", 123)]),
            And([("title", "*xyz"), ("brand_id", 456)]),
        )
    )
    .execute(db);
```

The `order_by()` method takes a tuple to specify the fields to be sorted and the
direction, or it can be transferred to a tuple vector to sort multiple fields.

```rust
// single field
omi::query::<Product>()
    .filter(("id", 123))
    .order_by(("id", Desc))
    .execute(db);

// multiple fields
omi::query::<Product>()
    .filter(("id", 123))
    .order_by([("id", Desc), ("id", Desc)])
    .execute(db);
```

And the group by operation is similar.

```rust
omi::query::<Product>().group_by(["brand_id")]).execute(db);
```

Finally, the `offset` and `limit` limits are essential for query statements

```rust
omi::query::<Product>().offset(0).limit(20).execute(db);
```

## Update

The first way is to make changes to the instance and then call the `save()` method
to update it. Omi will automatically recognize the model of this instance and
compare the differences, saving only the changed part of the fields to the
database.

```rust
omi::save(product).execute(db);
```

Of course, you can also call the `update()` method to update the specified fields
raw, which eliminates the need for prior data loading, reduces the number of
database reads, and, in certain cases, optimizes the processing speed of the
application.

```rust
// single field
omi::update::<Product>()
    .filter(("id", 123))
    .set("title", "abc")
    .execute(db);

// multiple fields
omi::update::<Product>()
    .filter(("id", 123))
    .set([("title", "abc"), ("brand_id", "456")])
    .execute(db);
```

## Delete

```rust
omi::delete(product).execute(db);
```

## Relations

You can call the `include()` or `exclude()` methods to include or exclude associated
objects, and by default all associated data for this model will be included.

```rust
omi::query::<Product>().include("reviews").execute(db);
omi::query::<Product>().exclude("reviews").execute(db);
```

## Transaction

```rust
omi::transaction.start();
omi::transaction.rollback();
omi::transaction.commit();
```

## Raw SQL Querying

```rust
omi::sql::<Product>(
    format!("SELECT * FROM products where id = {}", 123)
).execute(db);
```

## License

Released under the [Apache License 2.0](https://github.com/amphitheatre-app/amphitheatre/blob/master/LICENSE)
