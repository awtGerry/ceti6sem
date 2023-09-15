package com.example.examen;

import androidx.appcompat.app.AppCompatActivity;

import android.app.TimePickerDialog;
import android.content.Intent;
import android.os.Bundle;
import android.provider.AlarmClock;
import android.view.View;
import android.widget.TimePicker;

import java.util.Calendar;

public class CitaActivity extends AppCompatActivity {

    private final int ID = 1;
    private int hour, minute;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_cita);
    }

    public void programar_alarma(View view) {
        alarma("Cita", hour, minute);
    }

    public void setHourAlarm(View view) {
        Calendar calendar = Calendar.getInstance();
        int today_hour = calendar.get(Calendar.HOUR_OF_DAY);
        int today_minute = calendar.get(Calendar.MINUTE);
        TimePickerDialog tpg = new TimePickerDialog(CitaActivity.this, new TimePickerDialog.OnTimeSetListener() {
            @Override
            public void onTimeSet(android.widget.TimePicker timePicker, int i, int j) {
                hour = i;
                minute = j;
            }
        }, today_hour, today_minute, true);
        tpg.setTitle("Horario de la cita");
        tpg.show();
    }

    public void alarma(String msg, int h, int m) {
        Intent intent = new Intent(AlarmClock.ACTION_SET_ALARM)
                .putExtra(AlarmClock.EXTRA_MESSAGE, msg)
                .putExtra(AlarmClock.EXTRA_HOUR, h)
                .putExtra(AlarmClock.EXTRA_MINUTES, m);
        startActivity(intent);
    }
}