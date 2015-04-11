//
// Copyright 2015 Joshua R. Rodgers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

#[macro_use]
extern crate query_rs;

#[allow(unused_variables)]
#[test]
fn let_clause_should_introduce_new_context() {
    let result = query! { from x => (0..10).zip(0..10),
                          let (a, b) = x,
                          select a*b };

    let expected = vec! [0, 1, 4, 9, 16, 25, 36, 49, 64, 81];
 
    for (i, x) in result.enumerate() {
        assert_eq!(x, expected[i]);
    }
}

#[allow(unused_variables)]
#[test]
fn let_clause_should_be_able_to_take_closure() {
    let multiplier = 2;

    let result = query! { from x => (0..5),
                          let a = x * multiplier,
                          select a };

    let expected = vec! [0, 2, 4, 6, 8];
 
    for (i, x) in result.enumerate() {
        assert_eq!(x, expected[i]);
    }
}