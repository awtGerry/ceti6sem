package com.example.examen;

import androidx.appcompat.app.AppCompatActivity;
import androidx.core.app.NotificationCompat;

import android.os.Bundle;
import android.view.View;
import android.widget.EditText;
import android.widget.RadioButton;
import android.widget.Toast;

public class MainActivity extends AppCompatActivity {

    // OBJETO
    private Solicitud solicitud;

    private EditText curp, nombre, apellidos, domicilio, ingreso;
    private RadioButton medio, parcial, total;

    Notificaciones notificacion;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        // INICIAR ACTIVITY DE REGISTRO
        setContentView(R.layout.activity_registro);
        notificacion = new Notificaciones(this);

        // OBTENER DATOS
        curp = findViewById(R.id.id_curp);
        nombre = findViewById(R.id.id_nombre);
        apellidos = findViewById(R.id.id_apellido);
        domicilio = findViewById(R.id.id_domicilio);
        ingreso = findViewById(R.id.id_ingreso);
        medio = (RadioButton) findViewById(R.id.rb_medio);
        parcial = (RadioButton) findViewById(R.id.rb_parcial);
        total = (RadioButton) findViewById(R.id.rb_total);
    }

    public void validar(View view) {
        if (checkEmpty(curp) || checkEmpty(nombre) || checkEmpty(apellidos) || checkEmpty(domicilio) || checkEmpty(ingreso) || (!medio.isChecked() && !parcial.isChecked() && !total.isChecked())) {
            Toast.makeText(this, "Faltan campos por rellenar", Toast.LENGTH_SHORT).show();
        } else {
            solicitud = new Solicitud();
            solicitud.setCurp(curp.getText().toString());
            solicitud.setNombre(nombre.getText().toString());
            solicitud.setApellidos(apellidos.getText().toString());
            solicitud.setDomicilio(domicilio.getText().toString());
            solicitud.setIngreso(Double.parseDouble(ingreso.getText().toString()));
            if (medio.isChecked()) {
                solicitud.setTipo_credito("medio");
            } else if (parcial.isChecked()) {
                solicitud.setTipo_credito("parcial");
            } else if (total.isChecked()) {
                solicitud.setTipo_credito("total");
            }
            if (solicitud.validarIngreso()) {
                NotificationCompat.Builder builder = notificacion.crearNotificacion("Solicitud", "Su solicitud fue aceptada");
                notificacion.getNotificationManager().notify(1, builder.build());
            } else {
                Toast.makeText(this, "Su solicitud fue rechazada", Toast.LENGTH_SHORT).show();
            }
        }
    }

    private boolean checkEmpty(EditText editText) {
        return editText.getText().toString().isEmpty();
    }

    public void limpiar(View view) {
        curp.setText("");
        nombre.setText("");
        apellidos.setText("");
        domicilio.setText("");
        ingreso.setText("");
        medio.setChecked(false);
        parcial.setChecked(false);
        total.setChecked(false);
    }
}