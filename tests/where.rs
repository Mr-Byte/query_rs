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

#[test]
fn where_clause_should_remove_filtered_items() {
    let source = vec! [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let result = query! { from x => source.into_iter(),
                          where x % 2 == 0,
                          select x };

    let expected = vec! [2, 4, 6, 8, 10];
 
    for (i, x) in result.enumerate() {
        assert_eq!(x, expected[i]);
    }
}

#[test]
fn where_clause_should_be_able_to_take_closure() {
    let source = vec! [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let threshold = 3;
    
    let result = query! { from x => source.into_iter(),
                          where x > threshold,
                          select x };

    let expected = vec! [4, 5, 6, 7, 8, 9, 10];
 
    for (i, x) in result.enumerate() {
        assert_eq!(x, expected[i]);
    }
}