/**
 * TauriProvider — the in-process rumenx-chess engine.
 *
 * The window calls Tauri commands. There is no HTTP server.
 */

import { invoke } from '@tauri-apps/api/core';
import type {
  BackendId,
  ChessProvider,
  CreateGameOptions,
  NormAiMoveResult,
  NormAnalysis,
  NormGameState,
  NormLegalMove,
  NormMove,
  ProviderCapabilities,
} from './types';

export class TauriProvider implements ChessProvider {
  readonly backendId: BackendId = 'rust';

  readonly capabilities: ProviderCapabilities = {
    undo: true,
    ai: true,
    hint: true,
    analysis: true,
    chat: false,
    websocket: false,
    pgn: true,
    fenLoad: true,
  };

  dispose(): void {}

  async createGame(options?: CreateGameOptions): Promise<NormGameState> {
    return invoke<NormGameState>('new_game', { fen: options?.fen ?? null });
  }

  async getGame(gameId: string): Promise<NormGameState> {
    return invoke<NormGameState>('game_state', { gameId });
  }

  async deleteGame(gameId: string): Promise<void> {
    await invoke('delete_game', { gameId });
  }

  async makeMove(
    gameId: string,
    from: string,
    to: string,
    promotion?: string
  ): Promise<NormGameState> {
    return invoke<NormGameState>('make_move', {
      gameId,
      from,
      to,
      promotion: promotion ?? null,
    });
  }

  async undoMove(gameId: string): Promise<NormGameState> {
    return invoke<NormGameState>('undo', { gameId });
  }

  async getLegalMoves(gameId: string, fromSquare?: string): Promise<NormLegalMove[]> {
    return invoke<NormLegalMove[]>('legal_moves', {
      gameId,
      from: fromSquare ?? null,
    });
  }

  async aiMove(gameId: string, difficulty = 'medium'): Promise<NormAiMoveResult> {
    return invoke<NormAiMoveResult>('ai_move', { gameId, difficulty });
  }

  async aiHint(gameId: string, difficulty = 'easy'): Promise<NormMove> {
    return invoke<NormMove>('ai_hint', { gameId, difficulty });
  }

  async loadFen(gameId: string, fen: string): Promise<NormGameState> {
    return invoke<NormGameState>('load_fen', { gameId, fen });
  }

  async getPgn(gameId: string): Promise<string> {
    return invoke<string>('get_pgn', { gameId });
  }

  async getAnalysis(gameId: string, depth?: number): Promise<NormAnalysis> {
    return invoke<NormAnalysis>('get_analysis', { gameId, depth: depth ?? null });
  }
}
