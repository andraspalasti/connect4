package connect4;

import java.awt.Graphics;
import java.awt.Color;

import javax.swing.JPanel;

public class Board extends JPanel {
    @Override
    public void paint(Graphics g) {
      // super.paint(g);
      int w = getWidth();
      int h = getHeight();
      g.setColor(Color.BLACK);
      g.drawString(String.format("Width: %d, Height: %d", w, h), 50, 50);
    }
}
