package com.example.examen;

import android.app.NotificationChannel;
import android.app.NotificationManager;
import android.app.PendingIntent;
import android.app.TaskStackBuilder;
import android.content.Context;
import android.content.ContextWrapper;
import android.content.Intent;
import android.os.Build;

import androidx.core.app.NotificationCompat;

/* Clase para enviar notificaciones */
public class Notificaciones extends ContextWrapper {
    public static final String CHANNEL_ID = "CHANNELID";
    public static final String CHANNEL_NAME = "NOTIFICACION";

    private PendingIntent cita, beneficios;

    private NotificationManager manager;

    public Notificaciones(Context base) {
        super(base);
        crearCanal();
    }

    private void crearCanal() {
        /* Checar la version */
        NotificationChannel canal = new NotificationChannel(CHANNEL_ID, CHANNEL_NAME, NotificationManager.IMPORTANCE_DEFAULT);
        /* Configurar & registrar canal */
        canal.enableLights(true);
        canal.enableVibration(true);
        getNotificationManager().createNotificationChannel(canal);
        manager.createNotificationChannel(canal);
    }

    public NotificationManager getNotificationManager() {
        if (manager == null) {
            manager = (NotificationManager) getSystemService(Context.NOTIFICATION_SERVICE);
        }
        return manager;
    }

    /* Mandar a activity de alarma */
    private void setPendingIntentCita() {
        Intent intent = new Intent(this, CitaActivity.class);
        TaskStackBuilder stackBuilder = TaskStackBuilder.create(this);
        stackBuilder.addParentStack(CitaActivity.class);
        stackBuilder.addNextIntent(intent);
        cita = PendingIntent.getActivity(this, 0, intent, PendingIntent.FLAG_MUTABLE);
    }

    /* Mandar a activity de beneficios */
    private void setPendingIntentBeneficios() {
        Intent intent = new Intent(this, BeneficiosActivity.class);
        TaskStackBuilder stackBuilder = TaskStackBuilder.create(this);
        stackBuilder.addParentStack(BeneficiosActivity.class);
        stackBuilder.addNextIntent(intent);
        beneficios = PendingIntent.getActivity(this, 0, intent, PendingIntent.FLAG_MUTABLE);
    }

    public NotificationCompat.Builder crearNotificacion(String titulo, String msg) {
        setPendingIntentCita();
        setPendingIntentBeneficios();
        return new NotificationCompat.Builder(getApplicationContext(), CHANNEL_ID)
                .setContentTitle(titulo)
                .setContentText(msg)
                .setSmallIcon(R.drawable.ic_launcher_foreground)
                .setAutoCancel(true)
                .setPriority(NotificationCompat.PRIORITY_DEFAULT)
                .addAction(R.drawable.ic_launcher_foreground, "Cita", cita)
                .addAction(R.drawable.ic_launcher_foreground, "Beneficios", beneficios);
    }

}
