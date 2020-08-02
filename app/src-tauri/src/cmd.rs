use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
  Login(UserLogin),
  Register(UserRegister),
  GetUserData(i32),
  ChooseFolder,
  ClickedBtn,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserLogin {
    email: String,
    username: String,
    password: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserRegister {
    email: String,
    username: String,
    password: String,
}
