slint::include_modules!();
use std::rc::Rc;
use rand::seq::SliceRandom;
use slint::{Color, Model, Weak};

fn shuffle_tiles(app_ui_handle: &Weak<AppWindow>) -> Rc<slint::VecModel<TileData>> {
    let mut tiles: Vec<TileData> = app_ui_handle.unwrap().get_memory_tiles_data().iter().collect();
    tiles.extend(tiles.clone());
    let mut rng = rand::thread_rng();
    tiles.shuffle(&mut rng);

    let tiles_model = Rc::new(slint::VecModel::from(tiles));

    app_ui_handle.unwrap().set_memory_tiles_data(tiles_model.clone().into());

    return tiles_model;
}

fn check_pair(app_ui_handle: &Weak<AppWindow>, tiles_model: &Rc<slint::VecModel<TileData>>) {
    let mut flipped_tiles =
        tiles_model.iter().enumerate().filter(|(_, tile)| tile.image_visible && !tile.solved);

    if let (Some((t1_idx, mut t1)), Some((t2_idx, mut t2))) =
        (flipped_tiles.next(), flipped_tiles.next())
    {
        let is_pair_solved = t1 == t2;
        if is_pair_solved {
            t1.solved = true;
            tiles_model.set_row_data(t1_idx, t1);
            t2.solved = true;
            tiles_model.set_row_data(t2_idx, t2);
            set_app_text("Pair found !", Color::from_rgb_u8(0, 150, 0), app_ui_handle);
        } else {
            let app_ui_handle_clone = app_ui_handle.clone().unwrap().clone_strong();
            app_ui_handle_clone.set_disable_tiles(true);
            let tiles_model = tiles_model.clone();
            set_app_text("Try again !", Color::from_rgb_u8(200, 0, 0), app_ui_handle);
            slint::Timer::single_shot(std::time::Duration::from_secs(1), move || {
                app_ui_handle_clone.set_disable_tiles(false);
                t1.image_visible = false;
                tiles_model.set_row_data(t1_idx, t1);
                t2.image_visible = false;
                tiles_model.set_row_data(t2_idx, t2);
                set_app_text("", Color::from_rgb_u8(200, 0, 0), &app_ui_handle_clone.as_weak());
            });
        }
    }

    if tiles_model.iter().all(|tile| tile.solved) {
        set_app_text("You win !", Color::from_rgb_u8(0, 200, 0), app_ui_handle);
    }
}

fn set_app_text(text: &str, color: Color, app_ui_handle: &Weak<AppWindow>) {
    app_ui_handle.unwrap().set_current_text(text.to_string().into());
    app_ui_handle.unwrap().set_current_text_color(color.into());
}

fn main() -> Result<(), slint::PlatformError> {
    let app_ui = AppWindow::new()?;
    let app_ui_handle = app_ui.as_weak();

    let tiles_model = shuffle_tiles(&app_ui_handle);

    let app_ui_handle_clone = app_ui_handle.clone();
    app_ui_handle_clone.unwrap().on_check_if_pair_solved(move || {
        check_pair(&app_ui_handle_clone, &tiles_model);
    });

    set_app_text("Tap on a tile to reveal the image,\n find all pair to win the game !", Color::from_rgb_u8(0, 0, 0), &app_ui_handle);

    return app_ui.run();
}