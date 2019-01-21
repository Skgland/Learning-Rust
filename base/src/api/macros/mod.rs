#[macro_export]
macro_rules! part {
    ($name:ident in! $pattern:pat = $expr:expr) => {
        {
            let macro_result = $expr;
            if let $pattern = macro_result{
                $name
            }else{
                panic!("Failed to match refutable pattern irrefutable!")
            }
        };
    };
    ($name:ident in? $pattern:pat = $expr:expr) => {
        {
            let macro_result = $expr;
            if let $pattern = macro_result{
                $name
            }else{
                macro_result?
            }
        };
    };
    ($name:ident in $pattern:pat = { $expr:expr } else { $els:expr }) => {
        {
            let macro_result = $expr;
            if let $pattern = macro_result{
                $name
            }else{
                $els
            }
        };
    };
    ($name:ident try until $pattern:pat = $expr:expr ) => {
        {
            loop{
                if let $pattern = $expr{
                    break $name
                }
            }
        };
    };
}
