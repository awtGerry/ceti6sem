import java.awt.Color;
import java.awt.Graphics;

import javax.swing.*;

public class Casita extends JFrame {

    private final Color window_color = new Color(166, 227, 233);
    private final Color house_color = new Color(255, 255, 210);
    // private final Color roof_color = new Color(125, 90, 80);
    private final Color roof_color = new Color(170, 74, 68);
    private final Color grass_color = new Color(124, 252, 0);

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
        /* BACKGROUND */
        g.setColor(grass_color);
        g.fillRect(0, 320, 1366, 400);
        g.setColor(Color.CYAN);
        g.fillRect(0, 0, 1366, 368);

        /* SUN */
        g.setColor(Color.YELLOW);
        g.fillOval(1160, 50, 100, 100);

        /* HOUSE */
        g.setColor(house_color); // right side
        g.fillRect(600, 351, 321, 189);
        g.setColor(Color.BLACK); // right side container
        g.drawRect(600-1, 351, 321, 189);

        g.setColor(roof_color); // roof right side
        g.fillPolygon(new int[] {480, 500, 889, 949}, new int[]{361, 200, 200, 361}, 4);
        g.setColor(roof_color);
        g.fillRect(819, 180, 20, 30);
        g.setColor(roof_color); // left side roof
        g.fillPolygon(new int[] {380, 500, 620}, new int[] {300, 180, 300}, 3);

        g.setColor(house_color); // left side
        g.fillRect(400, 300, 200, 240);
        g.setColor(Color.BLACK); // container
        g.drawRect(399, 299, 201, 241);
        g.setColor(house_color); // second floor
        g.fillPolygon(new int[] {400, 500, 600}, new int[] {300, 200, 300}, 3);

        // Left side Windows
        g.setColor(Color.BLACK);
        g.drawRect(449, 419, 101, 61);
        g.setColor(window_color);
        g.fillRect(450, 420, 100, 60);
        g.setColor(Color.BLACK);
        g.drawLine(480, 480, 480, 420);
        g.setColor(Color.BLACK);
        g.drawLine(519, 480, 519, 420);

        g.setColor(Color.BLACK);
        g.drawRect(469, 289, 51, 51);
        g.setColor(window_color);
        g.fillRect(470, 290, 50, 50);
        g.setColor(Color.BLACK);
        g.drawLine(470, 315, 520, 315);
        g.setColor(Color.BLACK);
        g.drawLine(495, 290, 495, 340);

        // Right side window
        g.setColor(Color.BLACK);
        g.drawRect(800, 409, 61, 61);
        g.setColor(window_color);
        g.fillRect(801, 410, 60, 60);
        g.setColor(Color.BLACK);
        g.drawLine(830, 410, 830, 470);

        //door
        g.setColor(new Color(102, 51, 0));
        g.fillRect(670, 450, 52, 88);
        g.setColor(Color.YELLOW);
        g.fillOval(675, 490, 9, 9);
    }

}
