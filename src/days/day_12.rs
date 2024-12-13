use core::fmt;
use std::sync::Arc;

use axum::{
  extract::{Path, State},
  http::StatusCode,
  response::IntoResponse,
  routing::{get, post},
  Router,
};
use parking_lot::{Mutex, RwLock, RwLockUpgradableReadGuard};
use rand::{rngs::StdRng, Rng, SeedableRng};

static WALL: char = '⬜';
static EMPTY: char = '⬛';
static COOKIE: char = '🍪';
static MILK: char = '🥛';

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Cell {
  Empty,
  Cookie,
  Milk,
}

impl Default for Cell {
  fn default() -> Self {
    Self::Empty
  }
}

impl fmt::Display for Cell {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Empty => write!(f, "{EMPTY}"),
      Self::Cookie => write!(f, "{COOKIE}"),
      Self::Milk => write!(f, "{MILK}"),
    }
  }
}

#[derive(Default)]
struct Board {
  cells: [[Cell; 4]; 4],
  filled: u32,
}

impl fmt::Display for Board {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut res = String::new();

    for row in &self.cells {
      res.push(WALL);

      for el in row {
        res.push_str(&el.to_string());
      }

      res.push(WALL);
      res.push('\n');
    }

    res.push_str(&format!("{WALL}{WALL}{WALL}{WALL}{WALL}{WALL}\n"));

    match self.game_winner() {
      GameStatus::Winner(cell) => res.push_str(&format!("{cell} wins!\n")),
      GameStatus::Draw => res.push_str("No winner.\n"),
      GameStatus::Ongoing => {}
    }

    write!(f, "{res}")
  }
}

enum GameStatus {
  Winner(Cell),
  Draw,
  Ongoing,
}

impl Board {
  fn game_winner(&self) -> GameStatus {
    // Equal rows
    for row in &self.cells {
      if row.iter().all(|el| *el == row[0] && row[0] != Cell::Empty) {
        return GameStatus::Winner(row[0]);
      }
    }

    // Equal cols
    for i in 0..4 {
      let col = &self.cells.iter().map(|row| row[i]).collect::<Vec<_>>();

      if col.iter().all(|el| *el == col[0] && col[0] != Cell::Empty) {
        return GameStatus::Winner(col[0]);
      }
    }

    // Diagonal
    if self.cells[0][0] != Cell::Empty
      && self.cells[0][0] == self.cells[1][1]
      && self.cells[1][1] == self.cells[2][2]
      && self.cells[2][2] == self.cells[3][3]
    {
      return GameStatus::Winner(self.cells[0][0]);
    }
    if self.cells[3][0] != Cell::Empty
      && self.cells[3][0] == self.cells[2][1]
      && self.cells[2][1] == self.cells[1][2]
      && self.cells[1][2] == self.cells[0][3]
    {
      return GameStatus::Winner(self.cells[3][0]);
    }

    if self.filled == 16 {
      GameStatus::Draw
    } else {
      GameStatus::Ongoing
    }
  }

  fn is_col_ful(&self, col: usize) -> Option<bool> {
    if col >= 4 {
      return None;
    }

    Some(self.cells[0][col] != Cell::Empty)
  }

  fn is_game_over(&self) -> bool {
    match self.game_winner() {
      GameStatus::Draw | GameStatus::Winner(_) => true,
      GameStatus::Ongoing => false,
    }
  }

  fn generate_random(rand: &mut StdRng) -> Self {
    let mut res = Self::default();

    for i in 0..4 {
      for j in 0..4 {
        res.cells[i][j] = if rand.r#gen::<bool>() { Cell::Cookie } else { Cell::Milk };
      }
    }

    res
  }
}

#[derive(Clone)]
struct BoardState {
  board: Arc<RwLock<Board>>,
  rand: Arc<Mutex<StdRng>>,
}

pub fn get_routes() -> Router {
  let board = Arc::new(RwLock::new(Board::default()));
  let rand = Arc::new(Mutex::new(StdRng::seed_from_u64(2024)));

  Router::new()
    .route("/12/board", get(task_1a))
    .route("/12/reset", post(task_1b))
    .route("/12/place/:team/:column", post(task_2))
    .route("/12/random-board", get(task_3))
    .with_state(BoardState { board, rand })
}

async fn task_1a(state: State<BoardState>) -> String {
  state.board.read().to_string()
}

async fn task_1b(state: State<BoardState>) -> String {
  *state.board.write() = Board::default();
  *state.rand.lock() = StdRng::seed_from_u64(2024);

  state.board.read().to_string()
}

async fn task_2(
  state: State<BoardState>,
  Path((team, column)): Path<(String, usize)>,
) -> Result<String, impl IntoResponse> {
  let team = match team.as_str() {
    "cookie" => Ok(Cell::Cookie),
    "milk" => Ok(Cell::Milk),
    _ => Err(StatusCode::BAD_REQUEST.into_response()),
  }?;

  // Did the whole task with the assumption this was zero indexed
  // it was not
  let column = column
    .checked_sub(1)
    .and_then(|col| if col > 3 { None } else { Some(col) })
    .ok_or_else(|| StatusCode::BAD_REQUEST.into_response())?;

  let board = state.board.upgradable_read();

  if board.is_col_ful(column) == Some(true) || board.is_game_over() {
    return Err((StatusCode::SERVICE_UNAVAILABLE, board.to_string()).into_response());
  }

  for row in (0..=3).rev() {
    if board.cells[row][column] == Cell::Empty {
      let mut board = RwLockUpgradableReadGuard::upgrade(board);
      board.filled += 1;
      board.cells[row][column] = team;

      return Ok(board.to_string());
    }
  }

  Ok(String::new())
}

async fn task_3(state: State<BoardState>) -> String {
  let board = Board::generate_random(&mut state.rand.lock());
  let res = board.to_string();
  *state.board.write() = board;
  res
}
