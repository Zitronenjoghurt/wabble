use wabble_core::game::board::board_move::{BoardMove, BoardMovePart};
use wabble_core::game::board::coordinates::BoardCoords;
use wabble_core::game::board::tile::Tile;
use wabble_core::game::board::Board;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    let mut board = Board::new(15).unwrap();
    println!("{}", board.display_bonuses().unwrap());

    let p1 = BoardCoords::from_x_y(0, 0, board.size()).unwrap();
    let p2 = BoardCoords::from_x_y(1, 0, board.size()).unwrap();
    let p3 = BoardCoords::from_x_y(1, 1, board.size()).unwrap();
    board.get_cell_mut(&p1).unwrap().tile = Tile::A;
    board.get_cell_mut(&p2).unwrap().tile = Tile::A;
    board.get_cell_mut(&p3).unwrap().tile = Tile::A;

    let m1 = BoardCoords::from_x_y(0, 1, board.size()).unwrap();
    let m2 = BoardCoords::from_x_y(1, 1, board.size()).unwrap();
    let move_part1 = BoardMovePart::new(m1, Tile::A);
    let move_part2 = BoardMovePart::new(m2, Tile::A);
    let board_move = BoardMove::new(vec![move_part1, move_part2]);

    board.evaluate(&board_move).unwrap();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Wabble",
        native_options,
        Box::new(|cc| Ok(Box::new(wabble_app::WabbleApp::new(cc)))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;
    let web_options = eframe::WebOptions::default();

    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("app_canvas")
            .expect("Failed to find app_canvas")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("app_canvas was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(wabble_app::WabbleApp::new(cc)))),
            )
            .await;

        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}
