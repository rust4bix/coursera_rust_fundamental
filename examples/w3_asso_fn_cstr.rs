
struct User{
    username: String,
    email: String,
    uri: String,
    activate: bool, 
}

impl User{
    fn new(username: String, email: String, uri: String) -> Self{
        Self{
            username, 
            email,
            uri,
            activate: true
        }
    }

    fn deactivate(&mut self){
        self.activate = false; 
    }
}

fn main(){
    let new_user = User::new(
        String::from("Test"),
        String::from("Test@gmail.com"),
        String::from("http://Test"),
    );
    println!("Hell, {}", new_user.username); 
}