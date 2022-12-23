use bevy::{app::AppExit, prelude::*};
use iyes_loopless::prelude::*;

use crate::{despawn_with, rem, GameState};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::MainMenu, build_menu)
            .add_exit_system(GameState::MainMenu, despawn_with::<MainMenu>)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::MainMenu)
                    .with_system(button_visuals)
                    .with_system(start_game.run_if(on_click::<StartButton>))
                    .with_system(exit_game.run_if(on_click::<ExitButton>))
                    .into(),
            );
    }
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
struct StartButton;
#[derive(Component)]
struct ExitButton;

fn on_click<B: Component>(
    query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<B>)>,
) -> bool {
    for interaction in &query {
        if *interaction == Interaction::Clicked {
            return true;
        }
    }
    false
}

fn start_game(mut commands: Commands) {
    commands.insert_resource(NextState(GameState::Running));
}
fn exit_game(mut event_writer: EventWriter<AppExit>) {
    event_writer.send(AppExit);
}

fn button_visuals(
    mut query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color) in &mut query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

trait BuildButtons: BuildChildren {
    fn with_button(
        &mut self,
        asset_server: &Res<AssetServer>,
        text: impl Into<String>,
        bundle: impl Bundle,
    ) -> &mut Self {
        self.with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(rem!(8), rem!(4)),
                        margin: UiRect::all(rem!(0.5)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .insert(bundle)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        text,
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
        return self;
    }
}

impl<T: BuildChildren> BuildButtons for T {}

fn build_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    margin: UiRect::all(Val::Auto),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,

                    ..default()
                },
                ..default()
            },
            MainMenu,
        ))
        .with_button(&asset_server, "Start", StartButton)
        .with_button(&asset_server, "Exit", ExitButton);
}
