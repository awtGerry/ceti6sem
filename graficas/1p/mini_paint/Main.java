import java.awt.*;
import java.awt.event.*;
import javax.swing.*;

public class Main extends JFrame implements ActionListener, MouseListener, MouseMotionListener {

    private ButtonGroup modos;
    private JPanel area;
    private JLabel status;
    private Image buffer;
    private Image tmp;

    private final int puntos = 1;
    private final int lineas = 2;
    private final int rectangulos = 3;
    private final int circulos = 4;

    private int modo;
    private int x, y;

    public Main() {
        super("Mini paint");
        JMenuBar menuBar = new JMenuBar();
        JMenu menuArchivo = new JMenu("Archivo");
        JMenuItem opcionNuevo = new JMenuItem("Nuevo", 'N');
        opcionNuevo.addActionListener(this);
        opcionNuevo.setActionCommand("Nuevo");
        menuArchivo.add(opcionNuevo);

        menuArchivo.addSeparator();
        // Salir
        JMenuItem opcionSalir = new JMenuItem("Salir", 'S');
        opcionSalir.addActionListener(this);
        opcionSalir.setActionCommand("Salir");
        menuArchivo.add(opcionSalir);
        menuBar.add(menuArchivo);

        modos = new ButtonGroup();
        // Menu modo
        JMenu menuModo = new JMenu("Modo");

        /* Opciones */
        JRadioButtonMenuItem optPuntos = new JRadioButtonMenuItem("Puntos", true);
        optPuntos.addActionListener(this);
        optPuntos.setActionCommand("Puntos");
        menuModo.add(optPuntos);
        modos.add(optPuntos);

        JRadioButtonMenuItem optLineas = new JRadioButtonMenuItem("Lineas");
        optLineas.addActionListener(this);
        optLineas.setActionCommand("Lineas");
        menuModo.add(optLineas);
        modos.add(optLineas);

        // Rectangulos
        JRadioButtonMenuItem optRectangulos = new JRadioButtonMenuItem("Rectangulos");
        optRectangulos.addActionListener(this);
        optRectangulos.setActionCommand("Rectangulos");
        menuModo.add(optRectangulos);
        modos.add(optRectangulos);
        
        // Circulos
        JRadioButtonMenuItem optCirculos = new JRadioButtonMenuItem("Circulos");
        optCirculos.addActionListener(this);
        optCirculos.setActionCommand("Circulos");
        menuModo.add(optCirculos);
        modos.add(optCirculos);
        menuBar.add(menuModo);

        area = new JPanel();
        area.addMouseListener(this);
        area.addMouseMotionListener(this);
        status = new JLabel("Status: ", JLabel.LEFT);

        // Asignar barra para el menu
        setJMenuBar(menuBar);

        // Agregar zona grafica
        getContentPane().add(area, BorderLayout.CENTER);

        // Agregar barra de estado
        getContentPane().add(status, BorderLayout.SOUTH);
        modo = puntos;
        setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        setSize(400, 300);
        // show();
        buffer = area.createImage(area.getWidth(), area.getHeight());
    }

    public void actionPerformed(ActionEvent e) {
        String cmd = e.getActionCommand();
        if (cmd.equals("Salir")) {
            // dispose();
            System.exit(0);
        } else if (cmd.equals("Nuevo")) {
            area.getGraphics().clearRect(0, 0, area.getWidth(), area.getHeight());
        } else if (cmd.equals("Puntos")) {
            modo = puntos;
        } else if (cmd.equals("Lineas")) {
            modo = lineas;
        }
    }

    public void mouseClicked(MouseEvent e) {}

    public void mousePressed(MouseEvent e) {
        x = e.getX();
        y = e.getY();
        tmp = area.createImage(area.getWidth(), area.getHeight());
        tmp.getGraphics().drawImage(buffer, 0, 0, area);
    }

    public void mouseReleased(MouseEvent e) {
        buffer.getGraphics().drawImage(tmp, 0, 0, area);
    }

    public void mouseEntered(MouseEvent e) {
        setCursor(Cursor.getPredefinedCursor(Cursor.CROSSHAIR_CURSOR));
    }

    public void mouseExited(MouseEvent e) {
        setCursor(Cursor.getDefaultCursor());
    }

    public void mouseDragged(MouseEvent e) {
        Graphics g = tmp.getGraphics();
        switch (modo) {
            case puntos:
                g.fillOval(e.getX(), e.getY(), 1, 1);
                area.getGraphics().drawImage(tmp, 0, 0, this);
                break;
            case lineas:
                g.drawImage(buffer, 0, 0, area);
                g.drawLine(x, y, e.getX(), e.getY());
                area.getGraphics().drawImage(tmp, 0, 0, this);
                break;
            case rectangulos:
                g.drawImage(buffer, 0, 0, area);
                g.drawLine(x, y, e.getX()-x, e.getY()-y);
                area.getGraphics().drawImage(tmp, 0, 0, this);
                break;
            case circulos:
                g.drawImage(buffer, 0, 0, area);
                g.drawOval(x, y, e.getX()-x, e.getY()-y);
                area.getGraphics().drawImage(tmp, 0, 0, this);
                break;
        }
    }

    public void mouseMoved(MouseEvent e) {
        status.setText("Status: x=" + e.getX() + ", y=" + e.getY());
    }

    public static void main(String[] args) {
        Main window = new Main();
        window.setVisible(true);
    }

}
