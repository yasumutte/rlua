extern crate rlua;

use std::os::raw::c_void;

use rlua::{Function, LightUserData, Lua};

#[test]
fn test_lightuserdata() {
    Lua::new().context(|lua| {
        let globals = lua.globals();
        lua.exec::<_, ()>(
            r#"
                function id(a)
                    return a
                end
            "#,
            None,
        ).unwrap();
        let res = globals
            .get::<_, Function>("id")
            .unwrap()
            .call::<_, LightUserData>(LightUserData(42 as *mut c_void))
            .unwrap();
        assert_eq!(res, LightUserData(42 as *mut c_void));
    });
}
