use discord_game_sdk_sys as sys;
use std::os::raw::c_void;

pub unsafe extern "C" fn set_user_achievement(
    manager: *mut sys::IDiscordAchievementManager,
    achievement_id: sys::DiscordSnowflake,
    percent_complete: i64,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
    prevent_unwind!();
}

pub unsafe extern "C" fn fetch_user_achievements(
    manager: *mut sys::IDiscordAchievementManager,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
    prevent_unwind!();
}

pub unsafe extern "C" fn count_user_achievements(
    manager: *mut sys::IDiscordAchievementManager,
    count: *mut i32,
) {
    prevent_unwind!();
}

pub unsafe extern "C" fn get_user_achievement(
    manager: *mut sys::IDiscordAchievementManager,
    user_achievement_id: sys::DiscordSnowflake,
    user_achievement: *mut sys::DiscordUserAchievement,
) -> sys::EDiscordResult {
    prevent_unwind!();
    sys::DiscordResult_Ok
}

pub unsafe extern "C" fn get_user_achievement_at(
    manager: *mut sys::IDiscordAchievementManager,
    index: i32,
    user_achievement: *mut sys::DiscordUserAchievement,
) -> sys::EDiscordResult {
    prevent_unwind!();
    sys::DiscordResult_Ok
}