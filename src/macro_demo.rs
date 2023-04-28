macro_rules! string_enum {
  ($name:ident { $($variant:ident),* $(,)? }) => {
      // 生成枚举
      #[derive(Debug, PartialEq, Eq)]
      pub enum $name {
          $($variant),*
      }

      // 为枚举实现 `to_string` 方法
      impl $name {
          pub fn to_string(&self) -> &'static str {
              match self {
                  $(Self::$variant => stringify!($variant)),*
              }
          }
      }
  };
}

// 使用宏定义一个枚举
string_enum! {
  Direction {
      North,
      South,
      East,
      West,
  }
}

fn main() {
    let direction = Direction::North;
    println!("Direction: {:?}", direction);
    println!("Direction as string: {}", direction.to_string());
}
