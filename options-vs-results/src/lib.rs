#[cfg(test)]
mod tests {
    #[derive(Debug, Eq, PartialEq, Copy, Clone)]
    struct MyError;

    #[test]
    fn transposing() {
        // transpose could be used instead of map(), and_then()
        // the main point is switching between Ok(Some(x)) and Some(Ok(x))
        let this: Result<Option<i32>, MyError> = Ok(Some(42));
        let other: Option<Result<i32,MyError>> = Some(Ok(42));
        assert_eq!(this, other.transpose());

        // transpose lets us move between types
        assert_eq!(None::<Result<i32, MyError>>.transpose(), Ok(None::<i32>));
    }

    // this shows more traditional ways to convert between the 
    // two types including
    // - fetching the values of Ok and Err, and providing an error instance
    //  for positive results.
    //
    // the should generally be enough to replace most unwrap() or expect() calls and have
    // a single exec. path through the program without resorting to if/match conditionals.
    // positive: robustness + readability
    #[test]
    fn conversion() {
        // something that can be used instead of unwrap
        let opt = Some(42);
        assert_eq!(opt.ok_or(MyError), Ok(42));

        let res: Result<i32, MyError> = Ok(42);
        assert_eq!(res.ok(), opt);
        assert_eq!(res.err(), None);
        
        let opt: Option<i32> = None;
        assert_eq!(opt.ok_or(MyError), Err(MyError));

        let res: Result<i32, MyError> = Err(MyError);
        assert_eq!(res.ok(), None);
        assert_eq!(res.err(), Some(MyError));
    }
}
