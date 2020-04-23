use ent::{Ent, Result};

#[test]
fn test_serde_ops() {

    fn get_name<E:Ent>(e: &E) -> Result<&str> {
        e.get("name").unwrap().as_str()
    }

    let v: serde_json::Value = serde_json::from_str(r#"{"name":"test"}"#).unwrap();
    let ref name = get_name(&v).unwrap();
    assert_eq!(name, &"test")
}