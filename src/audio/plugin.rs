use bevy::prelude::*;

pub fn setup(app: &mut App) {
    app.add_message::<PlaySoundMessage>()
        .add_systems(Startup, load_audio_assets)
        .add_systems(Update, play_sound_system);
}

#[derive(Resource)]
struct AudioAssets {
    move_cursor: Handle<AudioSource>,
    start_game: Handle<AudioSource>,
    move_curr_piece: Handle<AudioSource>,
    rotate_curr_piece: Handle<AudioSource>,
    lock_curr_piece: Handle<AudioSource>,
    line_clear: Handle<AudioSource>,
    tetris_clear: Handle<AudioSource>,
    level_up: Handle<AudioSource>,
    game_over: Handle<AudioSource>,
}

fn load_audio_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(AudioAssets {
        move_cursor: asset_server.load("sounds/sfx02.ogg"),
        start_game: asset_server.load("sounds/sfx03.ogg"),
        move_curr_piece: asset_server.load("sounds/sfx04.ogg"),
        rotate_curr_piece: asset_server.load("sounds/sfx06.ogg"),
        lock_curr_piece: asset_server.load("sounds/sfx08.ogg"),
        line_clear: asset_server.load("sounds/sfx11.ogg"),
        tetris_clear: asset_server.load("sounds/sfx19.ogg"),
        level_up: asset_server.load("sounds/sfx07.ogg"),
        game_over: asset_server.load("sounds/sfx14.ogg"),
    });
}

#[derive(Message)]
pub enum PlaySoundMessage {
    MoveCursor,
    StartGame,
    MoveCurrPiece,
    RotateCurrPiece,
    LockCurrPiece,
    LineClear,
    TetrisClear,
    LevelUp,
    GameOver,
}

fn play_sound_system(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    mut play_sound: MessageReader<PlaySoundMessage>,
) {
    for event in play_sound.read() {
        let audio = match event {
            PlaySoundMessage::MoveCursor => &audio_assets.move_cursor,
            PlaySoundMessage::StartGame => &audio_assets.start_game,
            PlaySoundMessage::MoveCurrPiece => &audio_assets.move_curr_piece,
            PlaySoundMessage::RotateCurrPiece => &audio_assets.rotate_curr_piece,
            PlaySoundMessage::LockCurrPiece => &audio_assets.lock_curr_piece,
            PlaySoundMessage::LineClear => &audio_assets.line_clear,
            PlaySoundMessage::TetrisClear => &audio_assets.tetris_clear,
            PlaySoundMessage::LevelUp => &audio_assets.level_up,
            PlaySoundMessage::GameOver => &audio_assets.game_over,
        }
        .clone();
        commands.spawn((AudioPlayer(audio), PlaybackSettings::DESPAWN));
    }
}
