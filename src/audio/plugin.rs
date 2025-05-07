use bevy::prelude::*;

pub fn setup(app: &mut App) {
    app.add_event::<PlaySoundEvent>()
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

#[derive(Event)]
pub enum PlaySoundEvent {
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
    mut event_reader: EventReader<PlaySoundEvent>,
) {
    for event in event_reader.read() {
        let audio = match event {
            PlaySoundEvent::MoveCursor => audio_assets.move_cursor.clone(),
            PlaySoundEvent::StartGame => audio_assets.start_game.clone(),
            PlaySoundEvent::MoveCurrPiece => audio_assets.move_curr_piece.clone(),
            PlaySoundEvent::RotateCurrPiece => audio_assets.rotate_curr_piece.clone(),
            PlaySoundEvent::LockCurrPiece => audio_assets.lock_curr_piece.clone(),
            PlaySoundEvent::LineClear => audio_assets.line_clear.clone(),
            PlaySoundEvent::TetrisClear => audio_assets.tetris_clear.clone(),
            PlaySoundEvent::LevelUp => audio_assets.level_up.clone(),
            PlaySoundEvent::GameOver => audio_assets.game_over.clone(),
        };
        commands.spawn((AudioPlayer(audio), PlaybackSettings::DESPAWN));
    }
}
