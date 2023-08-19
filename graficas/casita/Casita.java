import java.awt.BorderLayout;
import java.awt.Color;
import java.awt.GridLayout;
import java.awt.Graphics;

import javax.swing.*;

public class Casita extends JFrame {
    private JFrame frame = new JFrame();
    private JPanel panel = new JPanel();
    private JLabel label = new JLabel();

    public Casita() {
        panel.setBorder(BorderFactory.createEmptyBorder(30, 30, 10, 30));
        panel.setLayout(new GridLayout(0, 1));
        panel.add(label);

        frame.add(panel, BorderLayout.CENTER);
        frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        frame.pack();
        // frame.setVisible(true);
    }

    public static void main(String[] args) {
        Casita casita = new Casita();
        casita.setVisible(true);
        // new Casita();
    }

    public void paint(Graphics g) {
        super.paint(g);
        // circulo, cuadrado, rectangulo
        g.drawOval(100, 100, 100, 100);
        g.fillOval(100, 100, 100, 100);
        g.setColor(Color.BLUE);
        g.drawRect(100, 200, 100, 100);
        g.fillRect(100, 200, 100, 100);
        g.setColor(Color.RED);
        g.drawRect(100, 300, 200, 100);
        g.fillRect(100, 300, 200, 100);
        g.setColor(Color.GREEN);
    }
}
