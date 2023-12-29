use napi::bindgen_prelude::{Buffer, Null, Object, Undefined};
use serde_json::Value;

/// the object to handle conversions
pub enum ObjectConvert {
  /// napi object
  Obj(Object),
  /// serde value
  Val(Value),
}

/// convert a napi object to json with trailing comma support for quick reading and writing
pub fn object_to_u8(obj: ObjectConvert) -> Result<Vec<u8>, napi::Error> {
  let mut ss = vec![];

  match obj {
    ObjectConvert::Val(deserialized) => {
      ss.extend(deserialized.to_string().as_bytes());
    }
    ObjectConvert::Obj(obj) => {
      let o = Object::keys(&obj)?;
      let o_size = o.len();

      ss.push(b'{');

      // we are missing map, null, and vector
      for (i, key) in o.iter().enumerate() {
        let mut fp = || {
          ss.push(b'"');
          ss.extend(key.as_bytes());
          ss.push(b'"');
          ss.push(b':');
        };

        let mut block = false;

        // todo: method to go through all napi values to get types instead of long chain map
        match obj.get::<&str, String>(&key) {
          Ok(s) => {
            fp();
            ss.push(b'"');
            ss.extend(s.unwrap_or_default().as_bytes());
            ss.push(b'"');
          }
          _ => match obj.get::<&str, u32>(&key) {
            Ok(s) => {
              fp();
              ss.push(b'"');
              ss.extend(s.unwrap_or_default().to_string().as_bytes());
              ss.push(b'"');
            }
            _ => match obj.get::<&str, i32>(&key) {
              Ok(s) => {
                fp();
                ss.push(b'"');
                ss.extend(s.unwrap_or_default().to_string().as_bytes());
                ss.push(b'"');
              }
              _ => match obj.get::<&str, Buffer>(&key) {
                Ok(s) => {
                  fp();
                  let d = serde_json::to_string(
                    &String::from_utf8(s.unwrap_or_default().as_ref().into()).unwrap_or_default(),
                  )?;
                  ss.extend(d.as_bytes());
                }
                _ => match obj.get::<&str, Null>(&key) {
                  Ok(_) => {
                    fp();
                    ss.extend(b"null");
                  }
                  _ => match obj.get::<&str, Undefined>(&key) {
                    Ok(_) => {
                      block = true;
                    }
                    _ => (),
                  },
                },
              },
            },
          },
        }

        if !block && i != o_size - 1 {
          ss.push(b',');
        }
      }

      ss.push(b'}');
    }
  }

  Ok(ss)
}
