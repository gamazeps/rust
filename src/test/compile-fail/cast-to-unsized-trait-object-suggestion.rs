// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn main() {
    &1 as Send;
    //~^ ERROR cast to unsized type
    //~| HELP try casting to a reference instead:
    //~| SUGGESTION &1 as &Send;
    Box::new(1) as Send;
    //~^ ERROR cast to unsized type
    //~| HELP try casting to a `Box` instead:
    //~| SUGGESTION Box::new(1) as Box<Send>;
}
