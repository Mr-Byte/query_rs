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
fn from_clause_should_allow_patterns() {
    let result = query! { from (a, b) => (0..5).zip(0..5),
                          select a * b };
                          
    let expected = [0, 1, 4, 9, 16];
    
    for (i, x) in result.enumerate() {
        assert_eq!(x, expected[i]);
    } 
}

#[test]
fn multiple_from_clauses_should_flat_map() {
    let input = [[0, 1], [2, 3], [4, 5]];

    let result = query! { from xs => input.iter(),
                          from &y => xs.iter(),
                          select y };

    let expected = vec! [0, 1, 2, 3, 4, 5];
 
    for (i, x) in result.enumerate() {
        assert_eq!(x, expected[i]);
    }
}

#[test]
fn an_operator_between_from_clauses_should_flat_map_the_result_of_the_operator() {
    let input = vec![vec! [0], vec! [1, 2], vec! [3, 4, 5]];

    let result = query! { from xs => input.iter(),
                          where xs.len() > 1,
                          from &y => xs.iter(),
                          select y };

    let expected = vec! [1, 2, 3, 4, 5];
 
    for (i, x) in result.enumerate() {
        assert_eq!(x, expected[i]);
    }
}