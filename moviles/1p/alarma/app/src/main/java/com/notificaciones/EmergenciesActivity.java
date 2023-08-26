package com.notificaciones;

import androidx.annotation.NonNull;
import androidx.appcompat.app.AppCompatActivity;
import androidx.core.app.NotificationManagerCompat;

import android.app.NotificationManager;
import android.content.Intent;
import android.os.Bundle;
import android.view.MenuItem;
import android.view.View;
import android.widget.ArrayAdapter;
import android.widget.EditText;
import android.widget.Spinner;
import android.widget.Toast;

import java.util.Objects;

public class EmergenciesActivity extends AppCompatActivity {

    private final String[] area = {
            "Cirugia",
            "Medicina Interna",
            "Ginecologia",
            "Obstetricia",
            "Traumatologia",
            "Oftalmologia",
            "Otorrinolaringologia",
            "Neurologia",
    };

    private final Integer[] roomNumber = {
            1,
            2,
            3,
            4,
            5,
    };

    private Spinner spinner, rooms;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_emergencies);
        Objects.requireNonNull(getSupportActionBar()).setDisplayHomeAsUpEnabled(true);
        NotificationManagerCompat notificationManager = NotificationManagerCompat.from(getApplicationContext());
        notificationManager.cancel(1);
        rooms = (Spinner) findViewById(R.id.id_rooms);
        spinner = (Spinner) findViewById(R.id.id_spinner);
        ArrayAdapter<String> adapter = new ArrayAdapter<String>(this, android.R.layout.simple_spinner_item, area);
        spinner.setAdapter(adapter);
        ArrayAdapter<Integer> adapter2 = new ArrayAdapter<Integer>(this, android.R.layout.simple_spinner_item, roomNumber);
        rooms.setAdapter(adapter2);
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

    public void sendPatientUrgencies(View view) {
        String area = spinner.getSelectedItem().toString();
        String room = rooms.getSelectedItem().toString();
        if (area.isEmpty() || room.isEmpty()) {
            Toast.makeText(this, "Se necesita mas informacion!", Toast.LENGTH_SHORT).show();
        } else {
            Toast.makeText(this, "Paciente registrado en habitacion " + room, Toast.LENGTH_SHORT).show();
        }
    }
}