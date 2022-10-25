use rouille::Request;
use crate::service::conf::User;

pub fn extract_authentication_info(request: &Request) -> Option<User> {
    let authorization = request.header("Authorization")?.split(" ").collect::<Vec<_>>()[1].to_string();
    let auth_bytes = base64::decode(authorization).ok()?;
    let auth = String::from_utf8(auth_bytes).ok()?.split(":").map(|s| s.to_string()).collect::<Vec<_>>();
    Some(User {
        user: auth[0].clone(),
        password: auth[1].clone(),
    })
}
