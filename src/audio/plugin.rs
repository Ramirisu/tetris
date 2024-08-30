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
        move_cursor: asset_server.load("sound/sfx02.wav"),
        start_game: asset_server.load("sound/sfx03.wav"),
        move_curr_piece: asset_server.load("sound/sfx04.wav"),
        rotate_curr_piece: asset_server.load("sound/sfx06.wav"),
        lock_curr_piece: asset_server.load("sound/sfx08.wav"),
        line_clear: asset_server.load("sound/sfx11.wav"),
        tetris_clear: asset_server.load("sound/sfx19.wav"),
        level_up: asset_server.load("sound/sfx07.wav"),
        game_over: asset_server.load("sound/sfx14.wav"),
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
        commands.spawn(AudioBundle {
            source: audio,
            settings: PlaybackSettings::DESPAWN,
        });
    }
}
