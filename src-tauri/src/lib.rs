mod games;

use std::sync::Mutex;

use games::{Games, NormAiMoveResult, NormAnalysis, NormGameState, NormMove};
use tauri::State;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(Games::new()))
        .invoke_handler(tauri::generate_handler![
            new_game,
            game_state,
            delete_game,
            legal_moves,
            make_move,
            undo,
            ai_move,
            ai_hint,
            load_fen,
            get_pgn,
            get_analysis
        ])
        .run(tauri::generate_context!())
        .expect("error while running chess-app");
}

type GameStore = Mutex<Games>;

#[tauri::command]
fn new_game(games: State<'_, GameStore>, fen: Option<String>) -> Result<NormGameState, String> {
    lock(&games)?.new_game(fen.as_deref())
}

#[tauri::command]
fn game_state(games: State<'_, GameStore>, game_id: String) -> Result<NormGameState, String> {
    lock(&games)?.game_state(&game_id)
}

#[tauri::command]
fn delete_game(games: State<'_, GameStore>, game_id: String) -> Result<(), String> {
    lock(&games)?.delete_game(&game_id);
    Ok(())
}

#[tauri::command]
fn legal_moves(
    games: State<'_, GameStore>,
    game_id: String,
    from: Option<String>,
) -> Result<Vec<NormMove>, String> {
    lock(&games)?.legal_moves(&game_id, from.as_deref())
}

#[tauri::command]
fn make_move(
    games: State<'_, GameStore>,
    game_id: String,
    from: String,
    to: String,
    promotion: Option<String>,
) -> Result<NormGameState, String> {
    lock(&games)?.make_move(&game_id, &from, &to, promotion.as_deref())
}

#[tauri::command]
fn undo(games: State<'_, GameStore>, game_id: String) -> Result<NormGameState, String> {
    lock(&games)?.undo(&game_id)
}

#[tauri::command]
async fn ai_move(
    games: State<'_, GameStore>,
    game_id: String,
    difficulty: Option<String>,
) -> Result<NormAiMoveResult, String> {
    let snapshot = lock(&games)?.clone_game(&game_id)?;
    let difficulty = difficulty.unwrap_or_else(|| "medium".to_string());
    let (mv, score, depth, elapsed) = tauri::async_runtime::spawn_blocking(move || {
        games::choose_move(&snapshot, &difficulty)
    })
    .await
    .map_err(|err| err.to_string())??;
    let mut result = lock(&games)?.apply_move(&game_id, mv)?;
    result.evaluation = score;
    result.depth = depth;
    result.thinking_time_ms = elapsed;
    Ok(result)
}

#[tauri::command]
async fn ai_hint(
    games: State<'_, GameStore>,
    game_id: String,
    difficulty: Option<String>,
) -> Result<NormMove, String> {
    let snapshot = lock(&games)?.clone_game(&game_id)?;
    let difficulty = difficulty.unwrap_or_else(|| "easy".to_string());
    let described = tauri::async_runtime::spawn_blocking(move || {
        let (mv, _, _, _) = games::choose_move(&snapshot, &difficulty)?;
        let mut described = games::describe_for_hint(mv);
        described.san =
            rumenx_chess::engine::san::move_to_san(snapshot.position(), mv, &snapshot.legal_moves());
        Ok::<_, String>(described)
    })
    .await
    .map_err(|err| err.to_string())??;
    Ok(described)
}

#[tauri::command]
fn load_fen(
    games: State<'_, GameStore>,
    game_id: String,
    fen: String,
) -> Result<NormGameState, String> {
    lock(&games)?.load_fen(&game_id, &fen)
}

#[tauri::command]
fn get_pgn(games: State<'_, GameStore>, game_id: String) -> Result<String, String> {
    lock(&games)?.pgn(&game_id)
}

#[tauri::command]
async fn get_analysis(
    games: State<'_, GameStore>,
    game_id: String,
    depth: Option<u32>,
) -> Result<NormAnalysis, String> {
    let snapshot = lock(&games)?.clone_game(&game_id)?;
    let depth = depth.unwrap_or(2).clamp(1, 5);
    let (mv, score, searched) = tauri::async_runtime::spawn_blocking(move || {
        let mut position = snapshot.position().clone();
        let (mv, stats) = rumenx_chess::ai::MinimaxAi::new().search(&mut position, depth);
        let mv = mv.ok_or_else(|| "the rust engine found no move".to_string())?;
        Ok::<_, String>((mv, stats.score, stats.depth))
    })
    .await
    .map_err(|err| err.to_string())??;
    Ok(NormAnalysis {
        evaluation: score,
        best_move: Some(games::describe_for_hint(mv)),
        depth: searched,
    })
}

fn lock<'a>(games: &'a State<'a, GameStore>) -> Result<std::sync::MutexGuard<'a, Games>, String> {
    games.lock().map_err(|err| err.to_string())
}
