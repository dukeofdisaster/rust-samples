#[cfg(test)]
mod tests {
    #[test]
    fn conditionals() {
      let i = 20;
      if i < 2 { 
        assert!(i < 2);
      } else if i > 2 {
        assert!(i > 2);
      } else {
        assert_eq!(i, 2);
      }
    }

    #[test]
    fn more_conditionals() {
      let my_option = Some(10);

      // if let statements can do pattern matching
      if let Some(unpacked) = my_option {
        assert_eq!(unpacked, 10);
      }

      let mut other_option = Some(2);

      //while let does the same thing
      while let Some(unpacked) = other_option { 
        // if can return values in assignments
        other_option = if unpacked > 0 { 
          Some(unpacked -1)
        } else { 
          None
        }
      }
      assert_eq!(other_option, None)
    }

    #[test]
    fn loops() {

      let mut i = 42;
      let mut broke = false;

      // basic loop 
      loop {
        i -= 1;
        if i < 2 { 
          broke = true;
          break;
        } else if i > 2 {
          continue;
        }
      }
      assert!(broke);

      // loops and other constructs can be named for readability
      'outer: loop { 
        'inner: loop {
          break 'inner; // and jumped out of...
        }
        break 'outer;
      }

      let mut iterations: u32 = 0;
      let total_squared = loop {
        iterations += 1; 
        if iterations >= 10 {
          // breaks on 10th iteration and returns 10.pow(2)
          break iterations.pow(2);
        }
      };
      assert_eq!(total_squared, 100);

      for i in 0..10 {
        assert!( i >= 0 && i < 10);
      }

      for v in vec![1,1,1,1].iter() { 
        assert_eq!(v, &1);
      }
    }
}
