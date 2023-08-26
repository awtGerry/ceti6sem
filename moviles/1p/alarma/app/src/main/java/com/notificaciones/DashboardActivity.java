package com.notificaciones;

import androidx.appcompat.app.AppCompatActivity;
import androidx.core.app.NotificationCompat;
import android.content.Intent;
import android.os.Bundle;
import android.view.View;
import android.widget.Button;

public class DashboardActivity extends AppCompatActivity {
    private NotificationHelper notificationHelper;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_dashboard);
        notificationHelper = new NotificationHelper(this);
        Button btnNotify = findViewById(R.id.button_register_patient);
        Button btnLogout = findViewById(R.id.button_logout);
        btnNotify.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                displayNotification();
            }
        });

        btnLogout.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                logout();
            }
        });
    }

    private void displayNotification() {
        NotificationCompat.Builder builder = notificationHelper.getChannelNotification("Paciente", "El paciente necesita atencion urgente?");
        notificationHelper.getManager().notify(1, builder.build());
    }

    private void logout() {
        Intent main = new Intent(this, MainActivity.class);
        startActivity(main);
    }
}