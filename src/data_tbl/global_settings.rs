use serde::deserialize;
use bevy::prelude::*;

// TODO: change this implimentation to use bevy common assets!
pub struct GlobalSettingsPlugin{}

impl Plugin for GlobalSettingsPlugin{
  fn build(&self, app: &mut App){
    app.add_systems(OnEnter(AppState::LoadingAssets), load_global_settings);
  }
}

#[derive(Deserialize, Debug, Clone)]
pub struct GlobalSettings{
  gravity_const: f32,
  velocity_max: f32,
  angular_velocity_max: f32,
}

impl GlobalSettings{
  /// 
  fn apply(&self){

  }
}

/*
impl Default for GlobalSettings{
  fn default() -> Self{
      GlobalSettings{
        gravity_const: 10.,
        velocity_max: 200.,
        angular_velocity_max: 20.,
      }
  }
}
*/

fn load_global_settings(){
  let path = "assets/global_settings.ron";
  let Some(g_settings) = fs::read_to_string(path) else{
    info!("failed to load global settings! Sticking to default.");
    commands.insert_resource(GlobalSettings::default();
      return;
    };
  commands.insert(g_settings);
}
