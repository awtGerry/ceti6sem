import javax.swing.*;
import java.awt.*;
import java.awt.event.*;

public class MiniPaint {

    public static void main(String[] args) {
        new Window().setVisible(true);
    }

    abstract static class Mode {
        public static final int POINT = 0;
        public static final int LINE = 1;
        public static final int CIRCLE = 2;
        public static final int RECTANGLE = 3;
    }

    static class Window extends JFrame {

        public Window() {
            super("MiniPaint 1.0");
            setSize(500, 500);
            setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
            setLocationRelativeTo(null);

            JMenuBar menuBar = new JMenuBar();

            JMenu fileMenu = new JMenu("Archivo");
            JMenuItem newMenuItem = new JMenuItem("Nuevo");

            fileMenu.add(newMenuItem);
            fileMenu.addSeparator();
            JMenuItem exitMenuItem = new JMenuItem("Salir");
            exitMenuItem.addActionListener(e -> {
                if (JOptionPane.showConfirmDialog(null, "¿Está seguro de que desea salir?", "Salir", JOptionPane.YES_NO_OPTION) == JOptionPane.YES_OPTION)
                    System.exit(0);
            });
            fileMenu.add(exitMenuItem);

            ButtonGroup options = new ButtonGroup();
            JMenu modeMenu = new JMenu("Modo");
            JRadioButtonMenuItem pointMode = new JRadioButtonMenuItem("Punto");
            JRadioButtonMenuItem lineMode = new JRadioButtonMenuItem("Línea");
            JRadioButtonMenuItem circleMode = new JRadioButtonMenuItem("Círculo");
            JRadioButtonMenuItem rectangleMode = new JRadioButtonMenuItem("Rectángulo");


            modeMenu.add(pointMode);
            modeMenu.add(lineMode);
            modeMenu.add(circleMode);
            modeMenu.add(rectangleMode);

            options.add(pointMode);
            options.add(lineMode);
            options.add(circleMode);
            options.add(rectangleMode);

            options.setSelected(pointMode.getModel(), true);

            menuBar.add(fileMenu);
            menuBar.add(modeMenu);

            add(menuBar, BorderLayout.NORTH);

            JLabel status = new JLabel("Status", JLabel.CENTER);
            Canvas canvas = new Canvas(status);
            getContentPane().add(canvas, BorderLayout.CENTER);

            pointMode.addActionListener(e -> canvas.setMode(Mode.POINT));
            lineMode.addActionListener(e -> canvas.setMode(Mode.LINE));
            circleMode.addActionListener(e -> canvas.setMode(Mode.CIRCLE));
            rectangleMode.addActionListener(e -> canvas.setMode(Mode.RECTANGLE));
            newMenuItem.addActionListener(e -> {
                if (JOptionPane.showConfirmDialog(null, "¿Está seguro de que desea crear un nuevo archivo?", "Nuevo", JOptionPane.YES_NO_OPTION) == JOptionPane.YES_OPTION) {
                    canvas.setImage(null);
                    getContentPane().repaint();
                }
            });
            getContentPane().add(status, BorderLayout.SOUTH);
        }

        static class Canvas extends JComponent {
            private Image image, imageTmp = createImage(getWidth(), getHeight());
            int mode = Mode.POINT;

            private int x, y;

            public void setMode(int mode) {
                this.mode = mode;
            }

            public void setImage(Image image) {
                this.image = image;
            }

            private int getMode() {
                return mode;
            }

            public Canvas(JLabel status) {
                addMouseListener(new MouseListener() {
                    @Override
                    public void mouseClicked(MouseEvent e) {
                    }

                    @Override
                    public void mousePressed(MouseEvent e) {
                        x = e.getX();
                        y = e.getY();
                        imageTmp = createImage(getWidth(), getHeight());
                        imageTmp.getGraphics().drawImage(image, 0, 0, null);
                    }

                    @Override
                    public void mouseReleased(MouseEvent e) {
                        image = createImage(getWidth(), getHeight());
                        image.getGraphics().drawImage(imageTmp, 0, 0, null);
                        imageTmp = null;
                        getGraphics().drawImage(image, 0, 0, null);
                    }

                    @Override
                    public void mouseEntered(MouseEvent e) {
                        setCursor(new Cursor(Cursor.CROSSHAIR_CURSOR));
                    }

                    @Override
                    public void mouseExited(MouseEvent e) {
                        setCursor(new Cursor(Cursor.DEFAULT_CURSOR));
                    }
                });

                addMouseMotionListener(new MouseMotionListener() {

                    @Override
                    public void mouseDragged(MouseEvent e) {
                        switch (getMode()) {
                            case Mode.POINT -> {
                                imageTmp.getGraphics().drawOval(e.getX(), e.getY(), 1, 1);
                                getGraphics().drawImage(imageTmp, 0, 0, null);
                            }
                            case Mode.LINE -> {
                                imageTmp.getGraphics().drawImage(image, 0, 0, null);
                                imageTmp.getGraphics().drawLine(x, y, e.getX(), e.getY());
                                getGraphics().drawImage(imageTmp, 0, 0, null);
                            }
                            case Mode.CIRCLE -> {
                                imageTmp.getGraphics().drawImage(image, 0, 0, null);
                                imageTmp.getGraphics().drawOval(x, y, e.getX() - x, e.getY() - y);
                                getGraphics().drawImage(imageTmp, 0, 0, null);
                            }
                            case Mode.RECTANGLE -> {
                                imageTmp.getGraphics().drawImage(image, 0, 0, null);
                                imageTmp.getGraphics().drawRect(x, y, e.getX() - x, e.getY() - y);
                                getGraphics().drawImage(imageTmp, 0, 0, null);
                            }
                        }
                    }

                    @Override
                    public void mouseMoved(MouseEvent e) {
                        status.setText("x=" + e.getX() + ", y=" + e.getY());
                    }
                });
            }
        }
    }
}
