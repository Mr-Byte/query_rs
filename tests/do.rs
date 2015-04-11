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
fn do_clauses_should_be_called_for_each_element_in_an_iterator() {
    const EXPECTED: i32 = 10;
    let mut call_count = 0;
    
    //Block to let rust know that we've finished borrowing call_count.
    {
        let iter = query! { from x => 0..10,
                            do call_count = call_count + 1,
                            select x };
                            
        for _ in iter {}
    }   
    
    assert_eq!(call_count, EXPECTED);
}