package connect4;

import javax.swing.*;
import java.awt.Dimension;

public class Main {
    public static void main(String[] args) {
        JFrame frame = new JFrame();

        Board board = new Board();
        frame.add(board);

        frame.setSize(500, 500);
        frame.setMinimumSize(new Dimension(500, 500));
        frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        frame.setVisible(true);
    }
}