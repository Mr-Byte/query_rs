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
fn where_let_clauses_should_filter_the_source_iterator() {
    let source = [Some(1), None, Some(3), None, Some(5)];
    
    let result = query! { from x => source.iter(),
                          where let Some(y) = x,
                          select y };
                          
    let expected = vec! [ 1, 3, 5 ];
    
    for (i, x) in result.enumerate() {
        assert_eq!(x, expected[i]);
    }
}

#[test]
fn where_let_clauses_should_allow_patterns_with_no_parameters() {
    let source = [Some(1), None, Some(3), None, Some(5)];
    
    let result = query! { from x => source.iter(),
                          where let None = x,
                          select x };
                          
    let expected = vec! [ None, None ];
    
    for (i, x) in result.enumerate() {
        assert_eq!(x, expected[i]);
    }
}