use crate::{
    namespace::NameSpace,
    identifier::Identifier,
    error::EvalError,
};

#[cfg(test)]
mod test_namespace {

    use super::*;

    #[test]
    fn test_declare() {
        let mut ns = NameSpace::new();
        let id = Identifier::from("x");
        assert!(ns.declare(&id, 42).is_ok());
        match ns.declare(&id, 42) {
            Ok(_) => panic!("Identifier should not be declared twice"),
            Err(EvalError::AlreadyDefined(id2)) => assert_eq!(id, id2),
            Err(_) => panic!("Unexpected error"),
        }
    }

    #[test]
    fn test_find() {
        let mut ns = NameSpace::new();
        let id = Identifier::from("x");
        match ns.find(&id) {
            Ok(_) => panic!("Identifier should not be found"),
            Err(EvalError::Undefined(id2)) => assert_eq!(id, id2),
            Err(_) => panic!("Unexpected error"),
        }
        assert!(ns.declare(&id, 42).is_ok());
        match ns.find(&id) {
            Ok(42) => (),
            _ => panic!("Identifier should be found"),
        }
    }

}
