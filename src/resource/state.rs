use crate::state::AppState;

const STATE: &str = "state";

pub async fn get_stored_state() -> Option<AppState> {
    let dir = dirs::data_local_dir().map(|d| {
        let mut path = d;
        path.push(super::ROOT);
        path.push(STATE);
        path
    });

    if let Some(dir) = dir {
        cacache::read(dir, STATE)
            .await
            .map(|d| bincode::deserialize::<AppState>(d.as_slice()).ok())
            .ok()
            .flatten()
    } else {
        None
    }
}

pub async fn store_state(state: AppState) {
    let dir = dirs::data_local_dir().map(|d| {
        let mut path = d;
        path.push(super::ROOT);
        path.push(STATE);
        path
    });

    if let Some(dir) = dir {
        _ = cacache::write(
            dir,
            STATE,
            bincode::serialize(&state).expect("serialize state"),
        )
        .await;
    }
}
