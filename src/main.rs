use bevy::prelude::*;

fn setup_camera(mut commands: Commands) {
    // commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // 프로그램 시작 시 카메라 설정 함수 실행
        .add_systems(Startup, setup_camera)
        .run();
}
