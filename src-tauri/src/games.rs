//! In-process chess games backed by `rumenx-chess`.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

use rumenx_chess::ai::{MinimaxAi, RandomAi};
use rumenx_chess::ai::engine::AiEngine;
use rumenx_chess::engine::pgn::to_pgn;
use rumenx_chess::engine::{Color, Game, GameStatus, Move, PieceType, Square};
use serde::Serialize;

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NormPiece {
    #[serde(rename = "type")]
    pub kind: String,
    pub color: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NormMove {
    pub from: String,
    pub to: String,
    pub san: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NormGameState {
    pub id: String,
    pub fen: String,
    pub turn: String,
    pub status: String,
    pub check: bool,
    pub move_count: usize,
    pub move_history: Vec<NormMove>,
    pub board: Vec<Vec<Option<NormPiece>>>,
    pub result: String,
    pub game_over: bool,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NormAiMoveResult {
    #[serde(rename = "move")]
    pub mv: NormMove,
    pub game: NormGameState,
    pub evaluation: i32,
    pub thinking_time_ms: u128,
    pub depth: u32,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NormAnalysis {
    pub evaluation: i32,
    pub best_move: Option<NormMove>,
    pub depth: u32,
}

pub struct Games {
    games: HashMap<String, Game>,
}

impl Games {
    pub fn new() -> Self {
        Self {
            games: HashMap::new(),
        }
    }

    pub fn new_game(&mut self, fen: Option<&str>) -> Result<NormGameState, String> {
        let game = match fen {
            Some(fen) if !fen.is_empty() => Game::from_fen(fen).map_err(|err| err.to_string())?,
            _ => Game::new(),
        };
        let id = format!("rust-{}", NEXT_ID.fetch_add(1, Ordering::Relaxed));
        let state = snapshot(&id, &game);
        self.games.insert(id, game);
        Ok(state)
    }

    pub fn game_state(&self, id: &str) -> Result<NormGameState, String> {
        self.get(id).map(|game| snapshot(id, game))
    }

    pub fn delete_game(&mut self, id: &str) {
        self.games.remove(id);
    }

    pub fn legal_moves(&self, id: &str, from: Option<&str>) -> Result<Vec<NormMove>, String> {
        let game = self.get(id)?;
        let from_sq = match from {
            Some(square) if !square.is_empty() => Some(parse_square(square)?),
            _ => None,
        };
        Ok(game
            .legal_moves()
            .into_iter()
            .filter(|mv| from_sq.is_none_or(|square| mv.from == square))
            .map(describe_move)
            .collect())
    }

    pub fn make_move(
        &mut self,
        id: &str,
        from: &str,
        to: &str,
        promotion: Option<&str>,
    ) -> Result<NormGameState, String> {
        let game = self.get_mut(id)?;
        let mv = find_move(game, from, to, promotion)?;
        game.make_move(mv).map_err(|err| err.to_string())?;
        Ok(snapshot(id, game))
    }

    pub fn undo(&mut self, id: &str) -> Result<NormGameState, String> {
        let game = self.get_mut(id)?;
        game.undo_move().map_err(|err| err.to_string())?;
        Ok(snapshot(id, game))
    }

    pub fn load_fen(&mut self, id: &str, fen: &str) -> Result<NormGameState, String> {
        let game = self.get_mut(id)?;
        game.load_fen(fen).map_err(|err| err.to_string())?;
        Ok(snapshot(id, game))
    }

    pub fn pgn(&self, id: &str) -> Result<String, String> {
        self.get(id).map(to_pgn)
    }

    pub fn clone_game(&self, id: &str) -> Result<Game, String> {
        self.get(id).cloned()
    }

    pub fn apply_move(&mut self, id: &str, mv: Move) -> Result<NormAiMoveResult, String> {
        let started = Instant::now();
        let game = self.get_mut(id)?;
        let before = describe_move(mv);
        game.make_move(mv).map_err(|err| err.to_string())?;
        let played = game.move_history().last().map_or(before.clone(), |record| NormMove {
            from: before.from.clone(),
            to: before.to.clone(),
            san: record.san.clone(),
            promotion: before.promotion.clone(),
        });
        Ok(NormAiMoveResult {
            mv: played,
            game: snapshot(id, game),
            evaluation: 0,
            thinking_time_ms: started.elapsed().as_millis(),
            depth: 0,
        })
    }

    fn get(&self, id: &str) -> Result<&Game, String> {
        self.games.get(id).ok_or_else(|| format!("game not found: {id}"))
    }

    fn get_mut(&mut self, id: &str) -> Result<&mut Game, String> {
        self.games
            .get_mut(id)
            .ok_or_else(|| format!("game not found: {id}"))
    }
}

pub fn choose_move(game: &Game, difficulty: &str) -> Result<(Move, i32, u32, u128), String> {
    let depth = search_depth(difficulty);
    let started = Instant::now();
    if depth == 0 {
        let mv = RandomAi
            .best_move(game, rumenx_chess::engine::Difficulty::Harmless)
            .map_err(|err| err.to_string())?;
        return Ok((mv, 0, 0, started.elapsed().as_millis()));
    }
    let mut position = game.position().clone();
    let (mv, stats) = MinimaxAi::new().search(&mut position, depth);
    let mv = mv.ok_or_else(|| "the rust engine found no move".to_string())?;
    Ok((mv, stats.score, stats.depth, started.elapsed().as_millis()))
}

pub fn search_depth(difficulty: &str) -> u32 {
    match difficulty {
        "harmless" => 0,
        "easy" => 1,
        "hard" => 3,
        "expert" => 4,
        "godlike" => 5,
        _ => 2,
    }
}

fn find_move(game: &Game, from: &str, to: &str, promotion: Option<&str>) -> Result<Move, String> {
    let from_sq = parse_square(from)?;
    let to_sq = parse_square(to)?;
    let want = promotion.and_then(promotion_piece);
    let matches: Vec<Move> = game
        .legal_moves()
        .into_iter()
        .filter(|mv| mv.from == from_sq && mv.to == to_sq)
        .collect();
    if matches.is_empty() {
        return Err(format!("illegal move {from}-{to}"));
    }
    if let Some(piece) = want {
        return matches
            .into_iter()
            .find(|mv| mv.promotion == Some(piece))
            .ok_or_else(|| format!("illegal promotion on {from}-{to}"));
    }
    Ok(matches
        .iter()
        .copied()
        .find(|mv| mv.promotion.is_none() || mv.promotion == Some(PieceType::Queen))
        .unwrap_or(matches[0]))
}

pub fn describe_for_hint(mv: Move) -> NormMove {
    describe_move(mv)
}

fn parse_square(square: &str) -> Result<Square, String> {
    Square::from_algebraic(square).ok_or_else(|| format!("bad square {square}"))
}

fn promotion_piece(raw: &str) -> Option<PieceType> {
    match raw.to_ascii_lowercase().as_str() {
        "q" | "queen" => Some(PieceType::Queen),
        "r" | "rook" => Some(PieceType::Rook),
        "b" | "bishop" => Some(PieceType::Bishop),
        "n" | "knight" => Some(PieceType::Knight),
        _ => None,
    }
}

fn describe_move(mv: Move) -> NormMove {
    NormMove {
        from: mv.from.to_algebraic(),
        to: mv.to.to_algebraic(),
        san: String::new(),
        promotion: mv.promotion.map(piece_name),
    }
}

fn snapshot(id: &str, game: &Game) -> NormGameState {
    let status = game.status();
    NormGameState {
        id: id.to_string(),
        fen: game.to_fen(),
        turn: game.side_to_move().to_string(),
        status: status.as_str().to_string(),
        check: matches!(status, GameStatus::Check | GameStatus::Checkmate),
        move_count: game.move_history().len(),
        move_history: game
            .move_history()
            .iter()
            .map(|record| NormMove {
                from: record.mv.from.to_algebraic(),
                to: record.mv.to.to_algebraic(),
                san: record.san.clone(),
                promotion: record.mv.promotion.map(piece_name),
            })
            .collect(),
        board: game
            .board_array()
            .into_iter()
            .map(|rank| rank.into_iter().map(|code| piece_from_code(&code)).collect())
            .collect(),
        result: result_of(game),
        game_over: game.is_game_over(),
    }
}

fn result_of(game: &Game) -> String {
    match game.status() {
        GameStatus::Checkmate => {
            if game.side_to_move() == Color::White {
                "0-1".to_string()
            } else {
                "1-0".to_string()
            }
        }
        GameStatus::Stalemate | GameStatus::Draw(_) => "1/2-1/2".to_string(),
        _ => "*".to_string(),
    }
}

fn piece_from_code(code: &str) -> Option<NormPiece> {
    let mut chars = code.chars();
    let color = match chars.next()? {
        'w' => "white",
        'b' => "black",
        _ => return None,
    };
    let kind = match chars.next()? {
        'P' => "pawn",
        'N' => "knight",
        'B' => "bishop",
        'R' => "rook",
        'Q' => "queen",
        'K' => "king",
        _ => return None,
    };
    Some(NormPiece {
        kind: kind.to_string(),
        color: color.to_string(),
    })
}

fn piece_name(piece: PieceType) -> String {
    match piece {
        PieceType::Queen => "queen",
        PieceType::Rook => "rook",
        PieceType::Bishop => "bishop",
        PieceType::Knight => "knight",
        PieceType::Pawn => "pawn",
        PieceType::King => "king",
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_game_reports_the_starting_position() {
        let mut games = Games::new();
        let state = games.new_game(None).unwrap();
        assert_eq!(
            state.fen,
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        );
        assert_eq!(state.turn, "white");
        assert!(!state.game_over);
    }

    #[test]
    fn e2e4_changes_the_side_to_move() {
        let mut games = Games::new();
        let state = games.new_game(None).unwrap();
        let state = games.make_move(&state.id, "e2", "e4", None).unwrap();
        assert_eq!(state.turn, "black");
        assert!(state.fen.starts_with("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b"));
    }

    #[test]
    fn an_illegal_move_is_rejected() {
        let mut games = Games::new();
        let state = games.new_game(None).unwrap();
        let err = games.make_move(&state.id, "e2", "e5", None).unwrap_err();
        assert!(err.contains("illegal"));
    }

    #[test]
    fn undo_restores_the_starting_position() {
        let mut games = Games::new();
        let state = games.new_game(None).unwrap();
        let starting = state.fen.clone();
        let state = games.make_move(&state.id, "e2", "e4", None).unwrap();
        let state = games.undo(&state.id).unwrap();
        assert_eq!(state.fen, starting);
        assert_eq!(state.move_count, 0);
    }

    #[test]
    fn play_a_move_then_an_ai_move_then_undo() {
        let mut games = Games::new();
        let state = games.new_game(None).unwrap();
        let id = state.id.clone();
        let state = games.make_move(&id, "e2", "e4", None).unwrap();
        assert_eq!(state.move_count, 1);
        let game = games.clone_game(&id).unwrap();
        let (mv, _, _, _) = choose_move(&game, "easy").unwrap();
        let result = games.apply_move(&id, mv).unwrap();
        assert_eq!(result.game.move_count, 2);
        assert_eq!(result.game.turn, "white");
        let state = games.undo(&id).unwrap();
        assert_eq!(state.move_count, 1);
        assert_eq!(state.turn, "black");
    }

    #[test]
    fn an_ai_move_on_the_starting_position_is_legal() {
        let mut games = Games::new();
        let state = games.new_game(None).unwrap();
        let game = games.clone_game(&state.id).unwrap();
        let legal: Vec<(String, String)> = game
            .legal_moves()
            .into_iter()
            .map(|mv| (mv.from.to_algebraic(), mv.to.to_algebraic()))
            .collect();
        assert_eq!(legal.len(), 20);
        let (mv, _, _, _) = choose_move(&game, "easy").unwrap();
        assert!(legal.contains(&(mv.from.to_algebraic(), mv.to.to_algebraic())));
    }
}
