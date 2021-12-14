use log::info;
use mogwai::prelude::*;

use crate::containers::home::Home;
use crate::containers::play::Play;

#[derive(Clone, Debug, PartialEq)]
pub enum Route {
    Home,
    Play,
}

impl Route {
    pub fn nav_home_class(&self) -> String {
        match self {
            Route::Home => "nav-link active",
            _ => "nav-link",
        }
        .to_string()
    }

    pub fn nav_play_class(&self) -> String {
        match self {
            Route::Play { .. } => "nav-link active",
            _ => "nav-link",
        }
        .to_string()
    }
}

impl TryFrom<&str> for Route {
    type Error = String;

    fn try_from(s: &str) -> Result<Route, String> {
        // remove the scheme, if it has one
        let hash_split = s.split("#").collect::<Vec<_>>();
        let after_hash = match hash_split.as_slice() {
            [_, after] => Ok(after),
            _ => Err(format!("route must have a hash: {}", s)),
        }?;

        let paths: Vec<&str> = after_hash.split("/").collect::<Vec<_>>();

        info!("pathss: {:?}, as_slice:{:?}", paths, paths.as_slice());
        match paths.as_slice() {
            [""] => Ok(Route::Home),
            ["", ""] => Ok(Route::Home),
            ["", "play"] => Ok(Route::Play),
            r => Err(format!("unsupported route: {:?}", r)),
        }
    }
}

impl From<Route> for String {
    fn from(route: Route) -> String {
        match route {
            Route::Home => "#/".into(),
            Route::Play => "#/play".into(),
        }
    }
}

impl From<&Route> for ViewBuilder<HtmlElement> {
    fn from(route: &Route) -> Self {
        match route {
            Route::Home => {
                let home_component = Gizmo::from(Home { ctx: None });

                builder! {
                    <main class="content">
                        {home_component.view_builder()}
                    </main>
                }
            }
            Route::Play => {
                let play_component = Gizmo::from(Play::default());
                builder! {
                    <main class="content">
                        {play_component.view_builder()}
                    </main>
                }
            }
        }
    }
}

impl From<&Route> for View<HtmlElement> {
    fn from(route: &Route) -> Self {
        ViewBuilder::from(route).into()
    }
}
