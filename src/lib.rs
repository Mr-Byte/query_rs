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

#[macro_export]
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

//This is a total hack and may need to be further tested with other patterns (i.e. does not work with nested patterns)
#[macro_export] 
macro_rules! query_where_let {
    ($source:expr, $context:pat => $t:ident($($newContext:ident),*) = $letValue:expr, $($remainder:tt)+) => {
        {
            let source = $source.filter_map(|&value| { 
                let $context = value; 
                if let $t($($newContext),*) = $letValue { 
                    Some((value, ($($newContext),*, ()))) 
                } 
                else {
                    None
                }
            });
            
            query!(source, ($context, ($($newContext),*, ())) => $($remainder)+)
        }
    };
    
    ($source:expr, $context:pat => $t:ident = $letValue:expr, $($remainder:tt)+) => {
        {
            let source = $source.filter_map(|&value| { 
                let $context = value; 
                if let $t = $letValue { 
                    Some(value) 
                } 
                else {
                    None
                }
            });
            
            query!(source, $context => $($remainder)+)
        }
    };
}
 
#[macro_export]
macro_rules! query_operator {
    ($source:expr, $context:pat => where let $($remainder:tt)+) => {
        query_where_let!($source, $context => $($remainder)+)
    };

    ($source:expr, $context:pat => where $filter:expr, $($remainder:tt)+) => {
        {
            let source = $source.filter(|&$context| { $filter });
            query!(source, $context => $($remainder)+)
        }
    };

    ($source:expr, $context:pat => let $newContext:pat = $letValue:expr, $($remainder:tt)+) => {
        {
            let source = $source.map(|value| { 
                let $context = value;
                (value, $letValue) 
            });
            
            query!(source, ($context, $newContext) => $($remainder)+)
        }
    };
    
    ($source:expr, $context:pat => do $action:stmt, $($remainder:tt)+) => {
        {
            let source = $source.inspect(|$context| { $action; });
            query!(source, $context => $($remainder)+)
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

#[macro_export]
macro_rules! query_end {
    ($source:expr, $context:pat => select $selector:expr) => {
        $source.map(|$context| { $selector })
    };
}