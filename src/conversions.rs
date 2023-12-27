use napi::bindgen_prelude::{Buffer, Null, Object, Undefined};

/// convert a napi object to json with trailing comma support for quick reading and writing
pub fn object_to_u8(
  obj: Object,
  collected_size: usize,
  inner_collected_size: usize,
) -> Result<Vec<u8>, napi::Error> {
  let o = Object::keys(&obj)?;
  let o_size = o.len();
  let mut ss = vec![];

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
    match obj.get::<&String, String>(&key) {
      Ok(s) => {
        fp();
        ss.push(b'"');
        ss.extend(s.unwrap_or_default().as_bytes());
        ss.push(b'"');
      }
      _ => match obj.get::<&String, u32>(&key) {
        Ok(s) => {
          fp();
          ss.push(b'"');
          ss.extend(s.unwrap_or_default().to_string().as_bytes());
          ss.push(b'"');
        }
        _ => match obj.get::<&String, i32>(&key) {
          Ok(s) => {
            fp();
            ss.push(b'"');
            ss.extend(s.unwrap_or_default().to_string().as_bytes());
            ss.push(b'"');
          }
          _ => match obj.get::<&String, Buffer>(&key) {
            Ok(s) => {
              fp();
              let d = serde_json::to_string(
                &String::from_utf8(s.unwrap_or_default().as_ref().into()).unwrap_or_default(),
              )?;
              ss.extend(d.as_bytes());
            }
            _ => match obj.get::<&String, Null>(&key) {
              Ok(_) => {
                fp();
                ss.extend(b"null");
              }
              _ => match obj.get::<&String, Undefined>(&key) {
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
  
  if collected_size > 0 && collected_size + 1 < inner_collected_size {
    ss.extend(b"\n");
  }

  Ok(ss)
}
