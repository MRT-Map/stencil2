use async_executor::{Executor, Task};
use bevy::prelude::*;
use egui_notify::ToastLevel;
use futures_lite::future;
use tracing::{error, info};

use crate::{
    component::skin::Skin,
    dirs_paths::cache_path,
    file::{load_msgpack, save_msgpack},
    misc_config::settings::INIT_MISC_SETTINGS,
    state::LoadingState,
    ui::notif::{NotifLogRwLockExt, NOTIF_LOG},
};

#[derive(Default)]
pub enum Step<T> {
    #[default]
    Uninitialised,
    Pending(Task<T>),
    Complete,
}

#[expect(clippy::cognitive_complexity)]
pub fn get_skin_sy(
    mut commands: Commands,
    mut task_s: Local<Step<surf::Result<Skin>>>,
    mut executor: Local<Option<Executor>>,
) {
    if cache_path("skin.msgpack").exists() {
        if let Ok(skin) = load_msgpack::<Skin>(&cache_path("skin.msgpack"), Some("skin")) {
            info!("Retrieved from cache");
            commands.insert_resource(skin);
            commands.insert_resource(NextState::Pending(LoadingState::LoadSkin.next()));
            *task_s = Step::Complete;
        }
    }
    let executor = executor.get_or_insert_with(Executor::new);
    match &mut *task_s {
        Step::Uninitialised => {
            let new_task = executor.spawn(async move {
                surf::get(&INIT_MISC_SETTINGS.skin_url)
                    .middleware(surf::middleware::Redirect::default())
                    .recv_json::<Skin>()
                    .await
            });
            info!("Retrieving skin");
            *task_s = Step::Pending(new_task);
        }
        Step::Pending(task) => match future::block_on(future::poll_once(task)) {
            None => {
                executor.try_tick();
            }
            Some(Ok(skin)) => {
                info!("Retrieved");
                let _ = save_msgpack(&skin, &cache_path("skin.msgpack"), Some("skin"));
                commands.insert_resource(skin);
                commands.insert_resource(NextState::Pending(LoadingState::LoadSkin.next()));
                *task_s = Step::Complete;
            }
            Some(Err(err)) => {
                error!(?err, "Unable to retrieve skin");
                NOTIF_LOG.push(format!("Couldn't download skin.\nMake sure you are connected to the internet.\nError: {err}"), ToastLevel::Error);
                *task_s = Step::Complete;
            }
        },
        Step::Complete => {}
    }
}
