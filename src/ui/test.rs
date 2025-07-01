use bevy::prelude::*;

fn spawn_box(mut commands: Commands){
  let container = Node{
    width: Val::Percent(100.),
    length: Val::Percent(100.),
    justify_context: JustifyContext::Center,
    ..default()
  };
    
  let square = Node{
    width: Val::Px(200.),
    border: UiRect::all(Val::Px(2.)),
    ..default()
  };

  let background_color = BackgroundColor(Color:::srgb(0.65, 0.65, 0.65));

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
