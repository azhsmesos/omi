// Copyright 2022 The Amphitheatre Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::entity::Entity;
use crate::{Ops, Statement};

pub trait Creatable<T> {
    /// Begin a insert session, which accepts an entity and returns a Statement
    /// instance
    fn create(entity: T) -> Statement<T>;
}

impl<T> Creatable<T> for T
where
    T: Entity,
{
    fn create(entity: T) -> Statement<T> {
        Statement::new(Ops::Insert)
    }
}

// #[cfg(test)]
// mod test {
//     use crate::{Entity, Ops};

//     #[Entity(table = "products")]
//     #[derive(Default)]
//     struct Product {
//         title: String,
//     }

//     #[test]
//     fn test_create() {
//         let session = Product::create(Product::default());
//         assert_eq!(session.ops, Ops::Insert);
//     }
// }