package com.example.examen;

public class Solicitud {
    private String curp, nombre, apellidos, domicilio, tipo_credito;
    private double ingreso;

    public Solicitud() {
    }

    public boolean validarIngreso(){
        if (this.tipo_credito.equals("medio")) {
            return this.ingreso >= 36000;
        }
        if (this.tipo_credito.equals("parcial")) {
            return this.ingreso >= 72000;
        }
        if (this.tipo_credito.equals("total")) {
            return this.ingreso >= 1120000;
        }
        return false;
    }

    public String getCurp() {
        return curp;
    }

    public void setCurp(String curp) {
        this.curp = curp;
    }

    public String getNombre() {
        return nombre;
    }

    public void setNombre(String nombre) {
        this.nombre = nombre;
    }

    public String getApellidos() {
        return apellidos;
    }

    public void setApellidos(String apellidos) {
        this.apellidos = apellidos;
    }

    public String getDomicilio() {
        return domicilio;
    }

    public void setDomicilio(String domicilio) {
        this.domicilio = domicilio;
    }

    public String getTipo_credito() {
        return tipo_credito;
    }

    public void setTipo_credito(String tipo_credito) {
        this.tipo_credito = tipo_credito;
    }

    public double getIngreso() {
        return ingreso;
    }

    public void setIngreso(double ingreso) {
        this.ingreso = ingreso;
    }
}
