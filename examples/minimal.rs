use std::time::Duration;

use bevy::{log::LogPlugin, prelude::*, time::common_conditions::on_timer};
use bevy_mod_reqwest::*;

fn send_requests(mut client: BevyReqwest) {
    let url = "https://bored-api.appbrewery.com/random";
    let req = client.get(url).build().unwrap();
    // will run the callback, and remove the created entity after callback
    client.send(req, |trigger: Trigger<ReqwestResponseEvent>| {
        let req = trigger.event();
        let res = req.as_str();
        let status = req.status();

        // let headers = req.response_headers();
        bevy::log::info!("code: {status}, data: {res:?}");
    });
}

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(LogPlugin::default())
        .add_plugins(ReqwestPlugin::default())
        .add_systems(
            Update,
            send_requests.run_if(on_timer(Duration::from_secs(2))),
        )
        .run();
}
