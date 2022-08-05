package connect4;

import javax.swing.*;
import java.awt.*;

public class Main {
    public static void main(String[] args) {
        JFrame frame = new JFrame();
        frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);

        Board board = new Board();
        board.setOpaque(true);
        frame.setContentPane(board);

        frame.setMinimumSize(new Dimension(500, 500));
        frame.pack();
        frame.setVisible(true);
    }
}