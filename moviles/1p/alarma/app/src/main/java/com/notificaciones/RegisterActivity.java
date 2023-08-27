package com.notificaciones;

import androidx.annotation.NonNull;
import androidx.appcompat.app.AppCompatActivity;
import androidx.core.app.NotificationCompat;
import androidx.core.app.NotificationManagerCompat;

import android.annotation.SuppressLint;
import android.app.DatePickerDialog;
import android.app.TimePickerDialog;
import android.content.Intent;
import android.os.Bundle;
import android.provider.AlarmClock;
import android.view.MenuItem;
import android.view.View;
import android.widget.Button;
import android.widget.DatePicker;
import android.widget.EditText;
import android.widget.Toast;

import java.util.Calendar;
import java.util.Objects;

public class RegisterActivity extends AppCompatActivity {

    private EditText name, lastname, age;
    private Button btn_date, btn_time;

    private final int alarm_id = 1;
    private int year, month, day, hour, minute;

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
    }

    public void registerPatient(View view) {
        String patientName = name.getText().toString();
        String patientLastname = lastname.getText().toString();
        String patientAge = age.getText().toString();
        if (patientName.isEmpty() || patientLastname.isEmpty() || patientAge.isEmpty()) {
            Toast.makeText(this, "Faltan campos por rellenar", Toast.LENGTH_SHORT).show();
        } else {
            NotificationHelper notificationHelper = new NotificationHelper(this);
            NotificationCompat.Builder builder = notificationHelper.getChannelNotification2("Paciente", "El paciente fue registrado, pronto sera atendido.");
            notificationHelper.getManager().notify(2, builder.build());
            programAlarm("Paciente listo para ingresar", hour, minute);
        }
    }

    public void setDateAlarm(View view) {
        Calendar calendar = Calendar.getInstance();
        int todayYear = calendar.get(Calendar.YEAR);
        int todayMonth = calendar.get(Calendar.MONTH);
        int todayDay = calendar.get(Calendar.DATE);
        DatePickerDialog datePickerDialog = new DatePickerDialog(RegisterActivity.this, new DatePickerDialog.OnDateSetListener() {
            @Override
            public void onDateSet(DatePicker datePicker, int i, int i1, int i2) {
                year = i;
                month = i1;
                day = i2;
            }
        }, todayYear, todayMonth, todayDay);
        datePickerDialog.setTitle("Fecha de la cita");
        datePickerDialog.show();
    }

    public void setHourAlarm(View view) {
        Calendar calendar = Calendar.getInstance();
        int todayHour = calendar.get(Calendar.HOUR_OF_DAY);
        int todayMinute = calendar.get(Calendar.MINUTE);
        TimePickerDialog timePickerDialog = new TimePickerDialog(RegisterActivity.this, new TimePickerDialog.OnTimeSetListener() {
            @Override
            public void onTimeSet(android.widget.TimePicker timePicker, int i, int i1) {
                hour = i;
                minute = i1;
            }
        }, todayHour, todayMinute, true);
        timePickerDialog.setTitle("Hora de la cita");
        timePickerDialog.show();
    }

    @SuppressLint("QueryPermissionsNeeded")
    public void programAlarm(String msg, int hr, int min) {
        Intent intent = new Intent(AlarmClock.ACTION_SET_ALARM)
                .putExtra(AlarmClock.EXTRA_MESSAGE, msg)
                .putExtra(AlarmClock.EXTRA_HOUR, hr)
                .putExtra(AlarmClock.EXTRA_MINUTES, min);
        startActivity(intent);
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