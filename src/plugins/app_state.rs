use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    MainMenu,
    LoadingScreen,
    #[default]
    InGame,
}

pub struct AppStatePlugin;

fn setup_menu() {
    //
}

impl Plugin for AppStatePlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.init_state::<AppState>()
            .add_systems(OnEnter(AppState::MainMenu), setup_menu);
    }
}
