package com.notificaciones;

import android.app.NotificationChannel;
import android.app.NotificationManager;
import android.app.PendingIntent;
import android.app.TaskStackBuilder;
import android.content.Context;
import android.content.ContextWrapper;
import android.content.Intent;

import androidx.core.app.NotificationCompat;

public class NotificationHelper extends ContextWrapper {
    public static final String channelID = "channelID";
    public static final String channelName = "Notificacion";
    public static final String channelID2 = "channelID2";
    public static final String channelName2 = "Notificacion2";

    private PendingIntent pendingIntentYes, pendingIntentNo;

    private NotificationManager mManager;

    public NotificationHelper(Context base) {
        super(base);
        createChannel();
    }

    private void createChannel() {
        if (android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.O) {
            NotificationChannel channel1 = new NotificationChannel(channelID, channelName, NotificationManager.IMPORTANCE_DEFAULT);
            channel1.enableLights(true);
            channel1.enableVibration(false);
            channel1.setLockscreenVisibility(android.app.Notification.VISIBILITY_PRIVATE);
            getManager().createNotificationChannel(channel1);
            NotificationChannel channel2 = new NotificationChannel(channelID2, channelName2, NotificationManager.IMPORTANCE_DEFAULT);
            channel2.setLockscreenVisibility(android.app.Notification.VISIBILITY_PRIVATE);
            getManager().createNotificationChannel(channel2);
        }
    }

    public NotificationManager getManager() {
        if (mManager == null) {
            mManager = (NotificationManager) getSystemService(Context.NOTIFICATION_SERVICE);
        }
        return mManager;
    }

    private void setPendingIntentYes() {
        Intent emergency = new Intent(this, EmergenciesActivity.class);
        TaskStackBuilder stackBuilder = TaskStackBuilder.create(this);
        stackBuilder.addParentStack(EmergenciesActivity.class);
        stackBuilder.addNextIntent(emergency);
        pendingIntentYes = PendingIntent.getActivity(this, 0, emergency, PendingIntent.FLAG_MUTABLE);
    }

    private void setPendingIntentNo() {
        Intent register = new Intent(this, RegisterActivity.class);
        TaskStackBuilder stackBuilder = TaskStackBuilder.create(this);
        stackBuilder.addParentStack(RegisterActivity.class);
        stackBuilder.addNextIntent(register);
        pendingIntentNo = PendingIntent.getActivity(this, 0, register, PendingIntent.FLAG_MUTABLE);
    }

    public NotificationCompat.Builder getChannelNotification(String title, String msg) {
        setPendingIntentYes();
        setPendingIntentNo();
        return new NotificationCompat.Builder(getApplicationContext(), channelID)
                .setContentTitle(title)
                .setContentText(msg)
                .setColor(getResources().getColor(R.color.red_500))
                .setAutoCancel(true)
                .setPriority(NotificationCompat.PRIORITY_DEFAULT)
                .addAction(R.drawable.ic_launcher_foreground, "Si", pendingIntentYes)
                .addAction(R.drawable.ic_launcher_foreground, "No", pendingIntentNo)
                .setSmallIcon(R.drawable.baseline_info_24);
    }
    public NotificationCompat.Builder getChannelNotification2(String title, String msg) {
        return new NotificationCompat.Builder(getApplicationContext(), channelID2)
                .setContentTitle(title)
                .setContentText(msg)
                .setPriority(NotificationCompat.PRIORITY_DEFAULT)
                .setAutoCancel(true)
                .setSmallIcon(R.drawable.baseline_info_24);
    }
}
