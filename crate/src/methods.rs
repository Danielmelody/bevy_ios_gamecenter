#![allow(unused_variables)]

#[allow(unused_imports)]
use crate::native;
use crate::{IosGCSaveGame, IosGCViewState};

/// Authenticate
/// Expected to be confirmed with `IosGamecenterEvents::Authentication` event
pub fn init() {
    if cfg!(target_os = "ios") {
        native::ios_gc_init();
    }
}

/// Request Player Infos
/// Expected to be confirmed with `IosGamecenterEvents::Player` event
///
/// ## Note
/// This will base64 encode the data to save it
pub fn request_player() {
    if cfg!(target_os = "ios") {
        native::get_player()
    }
}

/// Save Game under `name`
/// Expected to be confirmed with `IosGamecenterEvents::SavedGame` event
///
/// ## Note
/// This will base64 encode the data to save it
pub fn save_game(name: String, data: &[u8]) {
    if cfg!(target_os = "ios") {
        use base64::Engine;
        let s = base64::engine::general_purpose::STANDARD.encode(data);
        native::save_game(s, name);
    }
}

/// Requests the Data inside a given `IosGCSaveGame`
/// Expected to be confirmed with `IosGamecenterEvents::LoadGame` event
pub fn load_game(save_game: IosGCSaveGame) {
    if cfg!(target_os = "ios") {
        native::load_game(save_game);
    }
}

/// Requests a list of all available SaveGames
/// Expected to be confirmed with `IosGamecenterEvents::SaveGames` event
pub fn fetch_save_games() {
    if cfg!(target_os = "ios") {
        native::fetch_save_games();
    }
}

/// Updates progress on an Achievement.
/// Expected to be confirmed with `IosGamecenterEvents::AchievementProgress` event
pub fn achievement_progress(id: String, progress: f64) {
    if cfg!(target_os = "ios") {
        native::achievement_progress(id, progress);
    }
}

/// Resets all achievements.
/// Expected to be confirmed with `IosGamecenterEvents::AchievementsReset` event
pub fn achievements_reset() {
    if cfg!(target_os = "ios") {
        native::reset_achievements();
    }
}

/// Submits score to a leaderboard
/// Expected to be confirmed with `IosGamecenterEvents::LeaderboardScoreSubmitted` event
pub fn leaderboards_score(id: String, score: i64, context: i64) {
    if cfg!(target_os = "ios") {
        native::leaderboards_score(id, score, context);
    }
}

/// Opens Gamecenter View (to a specific `state`)
pub fn trigger_view(state: IosGCViewState) {
    if cfg!(target_os = "ios") {
        native::trigger_view(state);
    }
}
