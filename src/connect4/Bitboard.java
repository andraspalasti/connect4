package connect4;

import java.util.ArrayList;
import java.util.List;

/**
 * A bitboard for fast connect four operations. The implementation design came
 * from: https://github.com/denkspuren/BitboardC4/blob/master/BitboardDesign.md
 */
public class Bitboard {
    static final int WIDTH = 7;
    static final int HEIGHT = 6;

    /** Boards for each player */
    private long boards[] = { 0, 0 };

    /** The number of moves */
    private int moveCount = 0;

    /** The position to be filled in a column */
    private int fillPosition[] = { 0, 7, 15, 24, 30, 35, 42 };

    /** The moves that the players made during the game */
    private List<Integer> moves = new ArrayList<Integer>();

    /**
     * Makes a move
     * 
     * @param col The column to put the token in. Has to be inside [0, 6].
     */
    public void makeMove(int col) {
        long move = 1 << fillPosition[col]++;
        boards[moveCount & 1] ^= move;
        moves.add(col);
        moveCount++;
    }

    /**
     * Undos the previous move. There must be a previous move.
     */
    public void undoMove() {
        int col = moves.remove(--moveCount);
        long move = 1 << --fillPosition[col];
        boards[moveCount & 1] ^= move;
    }

    /**
     * Checks whether there are four connected tokens on the board.
     * 
     * @param bitboard The board to look
     * @return {@code true} if the board is winning else {@code false}
     */
    public boolean isWin(long bitboard) {
        if ((bitboard & (bitboard >> 6) & (bitboard >> 12) & (bitboard >> 18)) != 0)
            return true; // diagonal \
        if ((bitboard & (bitboard >> 8) & (bitboard >> 16) & (bitboard >> 24)) != 0)
            return true; // diagonal /
        if ((bitboard & (bitboard >> 7) & (bitboard >> 14) & (bitboard >> 21)) != 0)
            return true; // horizontal
        if ((bitboard & (bitboard >> 1) & (bitboard >> 2) & (bitboard >> 3)) != 0)
            return true; // vertical
        return false;
    }

    /**
     * Generate the moves that the current player can make
     * 
     * @return List of column indicies that you can put your token in
     */
    public List<Integer> listMoves() {
        List<Integer> moves = new ArrayList<Integer>();
        long TOP = 0b1000000_1000000_1000000_1000000_1000000_1000000_1000000L;
        for (int col = 0; col < fillPosition.length; col++) {
            if ((TOP & (1 << fillPosition[col])) == 0) {
                moves.add(col);
            }
        }
        return moves;
    }
}
