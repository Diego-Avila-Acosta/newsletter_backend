use unicode_segmentation::UnicodeSegmentation;

pub struct AdminPassword(String);

impl AdminPassword {
    pub fn new(password: String) -> Result<Self, ()> {
        let password_count = password.trim().graphemes(true).count();

        if password_count <= 12 {
            // TODO: Change to a better representation of error
            return Err(());
        }

        if password_count >= 129 {
            // TODO: Change to a better representation of error
            return Err(());
        }

        Ok(Self(password))
    }
}
