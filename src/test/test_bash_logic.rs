#[cfg(test)]
mod tests {
    use crate::command::{CommandRegistry, dispatch_line, register_commands, help, exit};

    #[test]
     fn registry_register_and_get() {
        let mut r = CommandRegistry::new();
        r.register("help", "Show every commands", Box::new(help::Help));

        assert!(r.get("help").is_some());
        assert!(r.get("nope").is_none());

        r.register("exit","Quit the game", Box::new(exit::Exit));

        assert!(r.get("exit").is_some());
        assert!(r.get("nope").is_none());
        
        assert_eq!(
         r.get_all(),
         vec![
            ("exit", "Quit the game"),
            ("help", "Show every commands"),
         ]);
     }

     #[test]
     fn dispatch_basic_and_alias() {
      let mut r = CommandRegistry::new();
      register_commands(&mut r);

      assert!(dispatch_line(&r, "help"));
      assert!(dispatch_line(&r, "h"));
      assert!(dispatch_line(&r, "?"));
      assert!(dispatch_line(&r, "aide"));

      assert!(!dispatch_line(&r, "q"));
      assert!(!dispatch_line(&r, "exit"));
      assert!(!dispatch_line(&r, "e"));

      assert!(dispatch_line(&r, "doesnotexist"));
      
     }
}