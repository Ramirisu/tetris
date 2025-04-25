use std::time::Duration;

use bevy::prelude::*;

pub fn despawn_all<T: Component>(query: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub fn format_hhmmss(duration: Duration) -> String {
    format!(
        "{:02}:{:02}:{:02}",
        duration.as_secs() / 3600,
        (duration.as_secs() / 60) % 60,
        duration.as_secs() % 60
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_hhmmss() {
        assert_eq!(format_hhmmss(Duration::from_secs(10)), "00:00:10");
        assert_eq!(format_hhmmss(Duration::from_secs(60)), "00:01:00");
        assert_eq!(format_hhmmss(Duration::from_secs(70)), "00:01:10");
        assert_eq!(format_hhmmss(Duration::from_secs(3600)), "01:00:00");
        assert_eq!(format_hhmmss(Duration::from_secs(3610)), "01:00:10");
        assert_eq!(format_hhmmss(Duration::from_secs(3670)), "01:01:10");
        assert_eq!(format_hhmmss(Duration::from_secs(86400)), "24:00:00");
        assert_eq!(format_hhmmss(Duration::from_secs(123456)), "34:17:36");
    }
}
