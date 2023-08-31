import javax.swing.*;

import java.awt.*;
import java.awt.image.BufferedImage;

public class Main extends JFrame {
    private Graphics graphics;
    private JFrame frame = new JFrame();
    BufferedImage buffer;

    private void createLine(int x1, int x2, int y1, int y2, Color color) {
        int size = 2;
        int dx = x2 - x1;
        int dy = y2 - y1;
        int steps = Math.max(Math.abs(dx), Math.abs(dy));
        float xInc = dx / (float) steps;
        float yInc = dy / (float) steps;
        float x = x1;
        float y = y1;
        for (int i=0; i<=steps; i++) {
            createPixel(Math.round(x), Math.round(y), size, color);
            x += xInc;
            y += yInc;
        }
    }

    private void createPixel(int x, int y, int size, Color color) {
        buffer = new BufferedImage(size, size, BufferedImage.TYPE_INT_RGB);
        for (int i=0; i<size; i++) {
            for (int j=0; j<size; j++) {
                buffer.setRGB(i, j, color.getRGB());
            }
        }
        this.graphics.drawImage(buffer, x, y, this);
    }

    public Main() {
        frame.setTitle("DDA algorithm");
        frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        frame.setSize(500, 500);
        frame.setResizable(false);
    }

    public void paint(Graphics g) {
        super.paint(g);
        graphics = g;
        Image bg = createImage(getWidth(), getHeight());
        g.drawImage(bg, 0, 0, this);
        createLine(0, 0, 500, 500, Color.BLACK);
        createLine(80, getWidth(), 900, 0, Color.GREEN);
        createLine(200, 200, getHeight(), 0, Color.BLUE);
        createLine(0, getWidth(), 400, 400, Color.BLUE);
    }

    public static void main(String[] args) {
        SwingUtilities.invokeLater(() -> new Main().setVisible(true));
    }
}
