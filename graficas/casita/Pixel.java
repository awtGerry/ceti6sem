import java.awt.*;
import java.awt.image.BufferedImage;

import javax.swing.*;

public class Pixel extends JFrame {
    private JFrame frame = new JFrame();
    private BufferedImage buffer;
    private Graphics pixel;

    public Pixel() {
        frame.setTitle("Ventana");
        frame.setSize(500, 500);
        frame.setLayout(null);
        buffer = new BufferedImage(20, 20, BufferedImage.TYPE_INT_RGB);
        pixel = (Graphics2D) buffer.getGraphics();
    }

    public void putPixel(int x, int y, Color c) {
        buffer.setRGB(10, 10, c.getRGB());
        this.getGraphics().drawImage(buffer, x, y, this);
    }

    public void paint(Graphics g) {
        super.paint(g);
        putPixel(100, 100, Color.RED);
    }

    public static void main(String[] args) {
        Pixel pixel = new Pixel();
        pixel.setVisible(true);
    }
}
