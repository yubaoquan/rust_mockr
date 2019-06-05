fn main() {
  let x = 5;
  let y = &x;
  let s = "xdfasdf";
  let s2 = String::from("xxxooo");
  let s3 = concat(&s2, &s2);

  // println!("Hello, world! {}, {}, {}, {}", x, y, &s, &s3);

  let mut clone = s2.clone();
  let result = add(&mut clone);
  println!("the result is {}, s2 is {}", result, &s2);
}


fn concat(str1: &str, str2: &str) -> String {
  return format!("{}---{}", str1, str2)
}

fn foo(s: &str) {
    println!("{}", s);
}

/// 拼接字符串
fn add(s: &mut String) -> &str {
  s.push_str("xxx");
  &s[..]
}
