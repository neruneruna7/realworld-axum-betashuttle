use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};

use crate::{
    endpoints::users::{
        dto::{NewUser, PasswdHashedNewUser},
        entity::UserEntity,
    },
    error::{ConduitResult, CustomArgon2Error},
};

pub struct PasswordHashService;

impl PasswordHashService {
    /// 成功した場合は何も返さない 失敗した場合はエラーを返す
    pub fn verify_password(
        stored_hashed_password: &str,
        attempt_password: &str,
    ) -> ConduitResult<()> {
        let expected = PasswordHash::new(stored_hashed_password).map_err(CustomArgon2Error)?;
        let argon2 = Argon2::default();
        argon2
            .verify_password(attempt_password.as_bytes(), &expected)
            .map_err(CustomArgon2Error)?;
        Ok(())
    }

    pub fn hash_password_newuser(req: NewUser) -> ConduitResult<PasswdHashedNewUser> {
        let hashed_pass = Self::hash_password(&req.password.unwrap()).map(|password| {
            PasswdHashedNewUser::new(req.username.unwrap(), req.email.unwrap(), password)
        })?;
        Ok(hashed_pass)
    }

    pub fn hash_password_user(user: UserEntity) -> ConduitResult<UserEntity> {
        let hashed_pass = Self::hash_password(&user.password).map(|password| UserEntity {
            email: user.email,
            username: user.username,
            password,
            bio: user.bio,
            image: user.image,
            ..user
        })?;
        Ok(hashed_pass)
    }

    fn hash_password(password: &str) -> ConduitResult<String> {
        let salt = SaltString::generate(&mut OsRng);
        // OWASPチートシートにより決定
        // https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html
        // let argon2 = Argon2::new(
        //     Algorithm::Argon2id,
        //     argon2::Version::V0x13,
        //     Params::new(19000, 2, 1, None).unwrap(),
        // );
        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(CustomArgon2Error)?;
        Ok(hash.to_string())
    }
}

#[cfg(test)]
mod tests {
    

    use super::*;

    #[test]
    fn hash_verify_password() {
        let password = "password";
        let hashed = PasswordHashService::hash_password(password).unwrap();
        println!("hashed: {:?}, from: {:?}", hashed, &password);
        PasswordHashService::verify_password(&hashed, password).unwrap();
    }

    #[test]
    fn hash_newuser_verify_password() {
        let new_user = NewUser {
            username: Some("username".to_string()),
            email: Some("email".to_string()),
            password: Some("password".to_string()),
        };
        let hashed = PasswordHashService::hash_password_newuser(new_user).unwrap();
        println!("hashed: {:?}, from: {:?}", hashed.password, "password");
        PasswordHashService::verify_password(&hashed.password, "password").unwrap();
    }
}
