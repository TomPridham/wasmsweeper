use crate::AppState;

use bevy::prelude::*;
use bevy_ui_build_macros::{build_ui, rect, size, style, unit};
use bevy_ui_navigation::{
    components::FocusableButtonBundle, FocusState, Focusable, NavEvent, NavEvent::NoChanges,
    NavMenu, NavRequest, NavRequest::Action,
};

#[derive(Clone, Component)]
struct MainMenuUI {}

fn button_focus_system(
    mut interaction_query: Query<(&Focusable, &mut UiColor), Changed<Focusable>>,
) {
    for (focus, mut material) in interaction_query.iter_mut() {
        let color = match focus.state() {
            FocusState::Focused => Color::ORANGE_RED,
            FocusState::Active => Color::GOLD,
            FocusState::Dormant => Color::WHITE,
            FocusState::Inert => Color::DARK_GRAY,
        };
        *material = color.into();
    }
}

fn start_button_system(
    mut app_state: ResMut<State<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Press".to_string();
                *color = Color::PURPLE.into();
                app_state.set(AppState::InGame).unwrap();
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = Color::RED.into();
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *color = Color::WHITE.into();
            }
        }
    }
}

fn menu_setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    use FlexDirection::ColumnReverse;

    let transparent: UiColor = Color::NONE.into();

    let vertical = NodeBundle {
        style: style! {
            flex_direction: ColumnReverse,
            size: size!(100 pct, 100 pct),
            margin: rect!(2 px),
        },
        color: transparent,
        ..Default::default()
    };
    let long = FocusableButtonBundle::from(ButtonBundle {
        style: style! {
            size: size!(100 pct, 40 px),
            margin: rect!(2 px),
        },
        color: Color::WHITE.into(),
        ..Default::default()
    });
    let start_text = TextBundle {
        text: Text::with_section(
            "Start",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 40.0,
                color: Color::LIME_GREEN,
            },
            Default::default(),
        ),
        ..Default::default()
    };

    let menu = || MainMenuUI {};
    build_ui! {
         #[cmd(commands)]
        menu(vertical{size:size!(100 pct, 100 pct)}[NavMenu::WrappingScope.root();](
            long(start_text)
        ))

    };
}

fn menu_cleanup(mut commands: Commands, query: Query<Entity, With<MainMenuUI>>) {
    for entity in query.iter() {
        // despawn the entity and its children
        commands.entity(entity).despawn_recursive();
    }
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(menu_setup));
        app.add_system_set(
            SystemSet::on_update(AppState::MainMenu)
                .with_system(button_focus_system)
                .with_system(start_button_system),
        );
    }
}
