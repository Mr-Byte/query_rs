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

#[macro_export]
macro_rules! query {
    ($($tokens:tt)*) => {
        query_from!($($tokens)*)
    };
}

macro_rules! query_from {
    (from $context:pat => $source:expr, $($remainder:tt)+) => {
        query_from!($source, $context => $($remainder)+)
    }; 
    
    ($source:expr, $context:pat => from $newContext:pat => $newSource:expr, $($remainder:tt)+) => {{
        let source = $source.flat_map(|value| {
            let $context = value;
            $newSource 
        });
        query_from!(source, $newContext => $($remainder)+)
    }};
    
    ($($tokens:tt)*) => {
        query_operator!($($tokens)*)
    };
}
 
macro_rules! query_operator {
    ($source:expr, $context:pat => where $filter:expr, $($remainder:tt)+) => {
        {
            let source = $source.filter(|&$context| { $filter });
            query!(source, $context => $($remainder)+)
        }
    };

    ($source:expr, $context:pat => let $newContext:pat = $letValue:expr, $($remainder:tt)+) => {
        {
            let source = $source.map(|value| { let $context = value; (value, $letValue) });
            query!(source, ($context, $newContext) => $($remainder)+)
        }
    };
    
    ($source:expr, $context:pat => from $($remainder:tt)+) => {
        {
            query_from!($source, $context => from $($remainder)+)
        }
    };
    
    ($($tokens:tt)*) => {
        query_end!($($tokens)*)
    };
}

macro_rules! query_end {
    ($source:expr, $context:pat => select $selector:expr) => {
        $source.map(|$context| { $selector })
    };
}

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
