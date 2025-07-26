use bevy::prelude::*;

use crate::{AppState, color_palette::PalColor};

pub struct TestPlugin {}
impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameReady), spawn_box);
    }
}
fn spawn_box(mut commands: Commands) {
    let container = Node {
        width: Val::Percent(10.),
        height: Val::Percent(20.),
        ..default()
    };

    let square = Node {
        width: Val::Px(200.),
        border: UiRect::all(Val::Px(2.)),
        ..default()
    };

    let background_color = BackgroundColor(PalColor::Blue.into());

    commands.spawn((container, children![(square, background_color)]));
}

fn spawn_text_in_ui(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        },
        Text::new("Here is some text"),
        TextColor(Color::BLACK),
        TextLayout::new_with_justify(JustifyText::Center),
    ));
}
