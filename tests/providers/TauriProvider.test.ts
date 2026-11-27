import { invoke } from '@tauri-apps/api/core';
import { TauriProvider } from '../../src/providers/TauriProvider';
import type { NormGameState } from '../../src/providers/types';

jest.mock('@tauri-apps/api/core', () => ({
  invoke: jest.fn(),
}));

const invokeMock = invoke as jest.MockedFunction<typeof invoke>;

const state: NormGameState = {
  id: 'rust-1',
  fen: 'rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1',
  turn: 'black',
  status: 'active',
  check: false,
  moveCount: 1,
  moveHistory: [{ from: 'e2', to: 'e4', san: 'e4' }],
  result: '*',
  gameOver: false,
};

describe('TauriProvider', () => {
  const provider = new TauriProvider();

  beforeEach(() => {
    invokeMock.mockReset();
  });

  it('maps a rust game into NormGameState', async () => {
    invokeMock.mockResolvedValue(state);

    const game = await provider.createGame();

    expect(invokeMock).toHaveBeenCalledWith('new_game', { fen: null });
    expect(game).toEqual(state);
    expect(game.id).toBe('rust-1');
    expect(game.turn).toBe('black');
    expect(game.moveHistory[0]).toEqual({ from: 'e2', to: 'e4', san: 'e4' });
  });

  it('sends a move to the rust command', async () => {
    invokeMock.mockResolvedValue(state);

    await provider.makeMove('rust-1', 'e2', 'e4');

    expect(invokeMock).toHaveBeenCalledWith('make_move', {
      gameId: 'rust-1',
      from: 'e2',
      to: 'e4',
      promotion: null,
    });
  });
});
