use bevy::{
    ecs::system::{EntityCommands, IntoObserverSystem},
    prelude::*,
};

pub struct CustomUIPlugin;
impl Plugin for CustomUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_button_updates);
    }
}

#[derive(Event, Debug)]
pub struct ButtonPressEvent;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub fn handle_button_updates(
    mut commands: Commands,
    mut interaction_query: Query<
        (Entity, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (entity, interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                commands.trigger_targets(ButtonPressEvent, entity);
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

pub trait UICommands {
    fn spawn_button<E: Event, B: Bundle, M>(
        &mut self,
        button_text: String,
        on_click: impl IntoObserverSystem<E, B, M>,
    ) -> EntityCommands;
    fn spawn_text(&mut self) -> EntityCommands;
}

impl<'w, 's> UICommands for Commands<'w, 's> {
    fn spawn_button<E: Event, B: Bundle, M>(
        &mut self,
        button_text: String,
        on_click: impl IntoObserverSystem<E, B, M>,
    ) -> EntityCommands {
        let mut entity_commands = self.spawn(ButtonBundle {
            style: Style {
                width: Val::Px(150.),
                height: Val::Px(65.),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: NORMAL_BUTTON.into(),
            ..default()
        });
        entity_commands.observe(on_click).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                button_text,
                TextStyle {
                    font_size: 40.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ));
        });
        entity_commands
    }
    fn spawn_text(&mut self) -> EntityCommands {
        todo!()
    }
}
