// Copyright 2019 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A point region in the tree.

// d8888b.  .d88b.  d888888b d8b   db d888888b
// 88  `8D .8P  Y8.   `88'   888o  88 `~~88~~'
// 88oodD' 88    88    88    88V8o 88    88
// 88~~~   88    88    88    88 V8o88    88
// 88      `8b  d8'   .88.   88  V888    88
// 88       `Y88P'  Y888888P VP   V8P    YP

use std::prelude::v1::*;
use {
    num::PrimInt,
    std::{
        fmt::Debug,
        ops::{Add, Sub},
    },
};

// Transparent alias. In docs and user-facing APIs, this resolves to (U, U).
pub(crate) type Type<U> = (U, U);

/// A type representing a point in space. Should be passed by value.
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct Point<U> {
    pub x: U, // The x-coordinate of the point.
    pub y: U, // The y-coordinate of the point.
}

impl<U> Debug for Point<U>
where
    U: PrimInt + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}x{:?}", self.x, self.y)
    }
}

impl<U> From<Type<U>> for Point<U>
where
    U: PrimInt,
{
    fn from((x, y): Type<U>) -> Self {
        Self { x, y }
    }
}

impl<U> From<&Type<U>> for Point<U>
where
    U: PrimInt,
{
    fn from((x, y): &Type<U>) -> Self {
        Self { x: *x, y: *y }
    }
}

impl<U> Into<Type<U>> for Point<U>
where
    U: PrimInt,
{
    fn into(self) -> Type<U> {
        (self.x, self.y)
    }
}

impl<U> Add for Point<U>
where
    U: PrimInt,
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x().saturating_add(other.x()),
            y: self.y().saturating_add(other.y()),
        }
    }
}

impl<U> Sub for Point<U>
where
    U: PrimInt,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x().saturating_sub(other.x()),
            y: self.y().saturating_sub(other.y()),
        }
    }
}

impl<U> Point<U>
where
    U: PrimInt,
{
    // pub

    /// The x-coordinate of the point.
    pub fn x(&self) -> U {
        self.x
    }

    /// The y-coordinate of the point.
    pub fn y(&self) -> U {
        self.y
    }
}
