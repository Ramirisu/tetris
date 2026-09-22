use bevy::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub fn plugin(app: &mut App) {
    app.add_message::<PlaySoundMessage>()
        .add_systems(Startup, load_audio_assets)
        .add_systems(Update, play_sound_system);
}

#[derive(Resource)]
struct AudioAssets {
    sounds: Vec<Handle<AudioSource>>,
}

fn load_audio_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(AudioAssets {
        sounds: PlaySoundMessage::iter()
            .map(|message| asset_server.load(message.asset_path()))
            .collect(),
    });
}

#[derive(Message, Clone, Copy, EnumIter)]
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

impl PlaySoundMessage {
    fn asset_index(&self) -> usize {
        *self as usize
    }

    fn asset_path(&self) -> &'static str {
        match self {
            Self::MoveCursor => "sounds/sfx02.ogg",
            Self::StartGame => "sounds/sfx03.ogg",
            Self::MoveCurrPiece => "sounds/sfx04.ogg",
            Self::RotateCurrPiece => "sounds/sfx06.ogg",
            Self::LockCurrPiece => "sounds/sfx08.ogg",
            Self::LineClear => "sounds/sfx11.ogg",
            Self::TetrisClear => "sounds/sfx19.ogg",
            Self::LevelUp => "sounds/sfx07.ogg",
            Self::GameOver => "sounds/sfx14.ogg",
        }
    }
}

fn play_sound_system(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    mut play_sound: MessageReader<PlaySoundMessage>,
) {
    for event in play_sound.read() {
        let audio = audio_assets.sounds[event.asset_index()].clone();
        commands.spawn((AudioPlayer(audio), PlaybackSettings::DESPAWN));
    }
}
