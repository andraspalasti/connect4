package connect4;

import java.awt.*;

import javax.swing.JPanel;

public class Board extends JPanel {
    Bitboard bitboard;

    public Board() {
        this.setPreferredSize(new Dimension(500, 500));
        bitboard = new Bitboard();
    }

    @Override
    public void paint(Graphics g) {
        super.paint(g);

        Graphics2D graphics2d = (Graphics2D) g;
        graphics2d.setRenderingHint(RenderingHints.KEY_ANTIALIASING, RenderingHints.VALUE_ANTIALIAS_ON);
        graphics2d.setBackground(Color.BLUE);

        int w = getWidth();
        int h = getHeight();

        graphics2d.setColor(Color.BLUE);
        graphics2d.fillRect(0, 0, w, h);

        int r = Math.min(Math.min(w / Bitboard.WIDTH, h / Bitboard.HEIGHT), 70);

        // Offsets are needed to center the board
        int offsetX = (w - r * Bitboard.WIDTH) / 2;
        int offsetY = (h - r * Bitboard.HEIGHT) / 2;

        final int PADDING = 18;
        for (int y = 0; y < Bitboard.HEIGHT; y++) {
            for (int x = 0; x < Bitboard.WIDTH; x++) {
                Token t = bitboard.getToken(x, y);
                switch (t) {
                    case RED:
                        graphics2d.setColor(Color.RED);
                        break;

                    case YELLOW:
                        graphics2d.setColor(Color.YELLOW);
                        break;

                    default:
                        graphics2d.setColor(Color.WHITE);
                        break;
                }

                graphics2d.fillOval(offsetX + x * r + PADDING / 2, offsetY + y * r + PADDING / 2,
                        r - PADDING,
                        r - PADDING);
            }
        }
    }
}
