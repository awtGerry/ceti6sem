import java.awt.Color;
import java.awt.Graphics;

import javax.swing.*;

public class Casita extends JFrame {

    public Casita() {
        setSize(1366, 768);
        setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
    }

    public static void main(String[] args) {
        Casita casita = new Casita();
        casita.setVisible(true);
    }

    public void paint(Graphics g) {
        super.paint(g);
        // BACKGROUND
        g.setColor(Color.GREEN);
        g.fillRect(0, 320, 1366, 400);
        g.setColor(Color.CYAN);
        g.fillRect(0, 0, 1366, 368);
        // SUN
        g.setColor(Color.YELLOW);
        g.fillOval(1160, 50, 100, 100);
        // HOUSE
        g.setColor(Color.LIGHT_GRAY);
        g.fillRect(600, 400, 200, 200);
        g.setColor(Color.LIGHT_GRAY);
        g.fillPolygon(new int[] { 600, 700, 800 }, new int[] { 400, 300, 400 }, 3);
        g.setColor(new Color(102, 51, 0));
        g.fillRect(670, 500, 60, 100);
        // Tree
        g.setColor(new Color(102, 51, 0));
        g.fillRect(100, 400, 60, 200);
        g.setColor(new Color(53, 94, 59));
        g.fillOval(50, 300, 150, 150);
    }

}
