use crate::{
    sys,
    utils::{charbuf_len, charbuf_to_str},
};

/// User Achievement
///
/// <https://discordapp.com/developers/docs/game-sdk/achievements#data-models-user-achievement-struct>
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Achievement {
    pub(crate) sys: sys::DiscordUserAchievement,
    unlocked_at_len: usize,
}

impl Achievement {
    /// The unique id of the user working on the achievement
    pub fn user_id(&self) -> i64 {
        self.sys.user_id
    }

    /// The unique id of the achievement
    pub fn achievement_id(&self) -> i64 {
        self.sys.achievement_id
    }

    /// How far along the user is to completing the achievement [0..=100]
    pub fn percent_complete(&self) -> u8 {
        self.sys.percent_complete
    }

    /// Date at which the user completed the achievement
    pub fn unlocked_at(&self) -> &str {
        charbuf_to_str(&self.sys.unlocked_at[..self.unlocked_at_len])
    }
}

impl From<sys::DiscordUserAchievement> for Achievement {
    fn from(sys: sys::DiscordUserAchievement) -> Self {
        Self {
            sys,
            unlocked_at_len: charbuf_len(&sys.unlocked_at),
        }
    }
}

impl std::fmt::Debug for Achievement {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Achievement")
            .field("user_id", &self.user_id())
            .field("achievement_id", &self.achievement_id())
            .field("percent_complete", &self.percent_complete())
            .field("unlocked_at", &self.unlocked_at())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::write_charbuf;

    #[test]
    fn test_unlocked_at() {
        run_test("");
        run_test("1");
        run_test("2");
        run_test("64 characters 64 characters 64 characters 64 characters 64 chara");
    }

    #[test]
    #[should_panic]
    fn panic_test_unlocked_at() {
        run_test("65 characters 65 characters 65 characters 65 characters 65 charac");
    }

    fn run_test(val: &str) {
        let mut source = sys::DiscordUserAchievement::default();

        write_charbuf(&mut source.unlocked_at, val);

        let achievement = Achievement::from(source);

        assert_eq!(achievement.unlocked_at(), val);
    }
}