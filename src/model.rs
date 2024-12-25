pub struct User {
     pub first_name: String,
     pub last_name: String,
     pub username: String,
     pub email: String,
     pub age: u8,
 }

 impl User {
     pub fn show_my_credential(&self) {
         println!(
             "first_name: {}, last_name: {}, username: {}, email: {}, age: {}",
             self.first_name, self.last_name, self.username, self.email, self.age
         )
     }
 }

 pub struct Weapon {
     pub name: String,
     pub magazine: u8,
     pub single_mode: bool
 }
