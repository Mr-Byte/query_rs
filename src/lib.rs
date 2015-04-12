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
macro_rules! context_match {
    ($value:expr, $context:pat => $result:expr) => {
        match $value {
            $context => $result,
            _ => panic!("Unexpected pattern context.")
        }
    }
}

#[macro_export]
macro_rules! query {
    ($($tokens:tt)*) => {
        query_from!($($tokens)*)
    };
}

#[macro_export]
macro_rules! query_from {
    (from $context:pat => $source:expr, $($remainder:tt)+) => {
        query_from!(match_none, $source, $context => $($remainder)+)
    }; 
    
    (match_none, $source:expr, $context:pat => from $newContext:pat => $newSource:expr, $($remainder:tt)+) => {{
        let source = $source.flat_map(|value| {
            let $context = value;
            $newSource 
        });
        
        query_from!(match_none, source, $newContext => $($remainder)+)
    }};
    
    //NOTE: In order to prevent "refutable pattern" errors potentially caused by where let,
    // I've added a Some(()) into the context to work around this issue.
    (match_context, $source:expr, $context:pat => from $newContext:pat => $newSource:expr, $($remainder:tt)+) => {{
        let source = $source.flat_map(|value| {
            context_match!(value, $context => $newSource.map(|value| { (value, Some(())) }))
        });
        
        query_from!(match_context, source, ($newContext, Some(())) => $($remainder)+)
    }};
    
    ($($tokens:tt)*) => {
        query_operator!($($tokens)*)
    };
}

#[macro_export]
macro_rules! query_where {
    (match_none, $source:expr, $context:pat => $filter:expr, $($remainder:tt)+) => {
        {
            let source = $source.filter(|&$context| { $filter });
            query!(match_none, source, $context => $($remainder)+)
        }
    };
    
    (match_context, $source:expr, $context:pat => $filter:expr, $($remainder:tt)+) => {
        {
            let source = $source.filter(|&value| { context_match!(value, $context => $filter) });
            query!(match_context, source, $context => $($remainder)+)
        }
    };
}

#[macro_export]
macro_rules! query_let {
    (match_none, $source:expr, $context:pat => $newContext:pat = $letValue:expr, $($remainder:tt)+) => {
        {
            let source = $source.map(|value| { 
                let $context = value;
                (value, $letValue) 
            });
            
            query!(match_none, source, ($context, $newContext) => $($remainder)+)
        }
    };
    
    (match_context, $source:expr, $context:pat => $newContext:pat = $letValue:expr, $($remainder:tt)+) => {
        {
            let source = $source.map(|value| { 
                context_match!(value, $context => (value, $letValue))
            });
            
            query!(match_context, source, ($context, $newContext) => $($remainder)+)
        }
    };
}

#[macro_export] 
macro_rules! query_where_let {
    (match_none, $source:expr, $context:pat => $newContext:pat = $letValue:expr, $($remainder:tt)+) => {
        {
            let source = $source.filter_map(|&value| { 
                let $context = value; 
                if let $newContext = $letValue { 
                    Some((value, $letValue)) 
                } 
                else {
                    None
                }
            });
            
            query!(match_context, source, ($context, $newContext) => $($remainder)+)
        }
    };
}
 
#[macro_export]
macro_rules! query_operator {
    ($match_context:ident, $source:expr, $context:pat => where let $($remainder:tt)+) => {
        query_where_let!($match_context, $source, $context => $($remainder)+)
    };

    ($match_context:ident, $source:expr, $context:pat => where $($remainder:tt)+) => {
        {
            query_where!($match_context, $source, $context => $($remainder)+)
        }
    };

    ($match_context:ident, $source:expr, $context:pat => let $($remainder:tt)+) => {
        {
            query_let!($match_context, $source, $context => $($remainder)+)
        }
    };
    
    ($match_context:ident, $source:expr, $context:pat => do $action:stmt, $($remainder:tt)+) => {
        {
            let source = $source.inspect(|$context| { $action; });
            query!($match_context, source, $context => $($remainder)+)
        }
    };
    
    ($match_context:ident, $source:expr, $context:pat => from $($remainder:tt)+) => {
        {
            query_from!($match_context, $source, $context => from $($remainder)+)
        }
    };  

    ($($tokens:tt)*) => {
        query_end!($($tokens)*)
    };
}

#[macro_export]
macro_rules! query_end {    
    (match_context, $source:expr, $context:pat => select $selector:expr) => {
        $source.map(|value| { context_match!(value, $context => $selector) })
    };

    (match_none, $source:expr, $context:pat => select $selector:expr) => {
        $source.map(|$context| { $selector })
    };
}