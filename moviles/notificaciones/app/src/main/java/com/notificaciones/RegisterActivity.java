package com.notificaciones;

import androidx.annotation.NonNull;
import androidx.appcompat.app.AppCompatActivity;
import androidx.core.app.NotificationCompat;
import androidx.core.app.NotificationManagerCompat;

import android.content.Intent;
import android.os.Bundle;
import android.view.MenuItem;
import android.view.View;
import android.widget.EditText;
import android.widget.Toast;

import java.util.Objects;

public class RegisterActivity extends AppCompatActivity {

    private EditText name, lastname, age, date;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_register);
        Objects.requireNonNull(getSupportActionBar()).setDisplayHomeAsUpEnabled(true);
        NotificationManagerCompat notificationManager = NotificationManagerCompat.from(getApplicationContext());
        notificationManager.cancel(1);
        name = (EditText) findViewById(R.id.id_patient_name);
        lastname = (EditText) findViewById(R.id.id_patient_lastname);
        age = (EditText) findViewById(R.id.id_patient_age);
        date = (EditText) findViewById(R.id.id_date);
    }

    public void registerPatient(View view) {
        String patientName = name.getText().toString();
        String patientLastname = lastname.getText().toString();
        String patientAge = age.getText().toString();
        String patientDate = date.getText().toString();
        if (patientName.isEmpty() || patientLastname.isEmpty() || patientAge.isEmpty() || patientDate.isEmpty()) {
            Toast.makeText(this, "Faltan campos por rellenar", Toast.LENGTH_SHORT).show();
        } else {
            NotificationHelper notificationHelper = new NotificationHelper(this);
            NotificationCompat.Builder builder = notificationHelper.getChannelNotification2("Paciente", "El paciente fue registrado, pronto sera atendido.");
            notificationHelper.getManager().notify(2, builder.build());
        }
    }

    @Override
    public boolean onOptionsItemSelected(@NonNull MenuItem item) {
        if (item.getItemId() == android.R.id.home) {
            Intent dashboard = new Intent(this, DashboardActivity.class);
            startActivity(dashboard);
            finish();
            return true;
        }
        return super.onOptionsItemSelected(item);
    }
}