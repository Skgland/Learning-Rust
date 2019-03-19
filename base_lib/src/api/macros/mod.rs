
///
/// The part Macro tries to match a Pattern
/// if successful it will return the part matching name
/// if unsuccessful it will either
///
///  - `!          =>` panic! with the error message `"Failed to match refutable pattern irrefutably!"`
///  - `!$erm:expr =>` panic with the message specified in $erm
///  - `?            ` try to apply the ? operator to the result of $expr
///  - `in           ` return $els instead
///  - `try until    ` retry to evaluate $expr against the pattern
///
///

#[macro_export]
macro_rules! part {
    ($name:ident in ($pattern:pat = $expr:expr) else ?) => {
        {
            let macro_result = $expr;
            if let $pattern = macro_result{
                $name
            }else{
                macro_result?
            }
        };
    };
    ($name:ident in ($pattern:pat = $expr:expr) else retry) => {
        {
            loop{
                if let $pattern = $expr{
                    break $name
                }
            }
        };
    };
    ($name:ident in ($pattern:pat = $expr:expr) else $els:expr ) => {
        {
            let macro_result = $expr;
            if let $pattern = macro_result{
                $name
            }else{
                $els
            }
        };
    };
}
