// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

trait Foo {
    fn foo<T: Default>(x: T) -> Self; //~ NOTE expected 1 type parameter
}

struct Bar;

impl Foo for Bar {
    fn foo(x: bool) -> Self { Bar } //~ ERROR E0049
                                    //~| NOTE found 0 type parameters
}

fn main() {
}
