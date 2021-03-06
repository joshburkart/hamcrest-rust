// Copyright 2017 Povilas Balciunas
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::*;

impl Matcher<bool> for bool {
    fn matches(&self, actual: bool) -> MatchResult {
        if actual == *self {
            success()
        } else {
            Err(format!("was {:?}", actual))
        }
    }
}
