package com.example.multimedia;

import androidx.appcompat.app.AppCompatActivity;

import android.media.AudioManager;
import android.media.MediaPlayer;
import android.net.Uri;
import android.os.Bundle;
import android.view.View;
import android.widget.Button;
import android.widget.ImageButton;
import android.widget.Toast;

import java.io.IOException;

public class ActivityMediaPlayer extends AppCompatActivity {
    ImageButton play, stop;
    MediaPlayer mediaPlayer;
    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_media_player);

        play = findViewById(R.id.btnMediaPlay);
        stop = findViewById(R.id.btnMediaStop);

        play.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View view) {
                mediaPlayer = new MediaPlayer();
                mediaPlayer.setAudioStreamType(AudioManager.STREAM_MUSIC);
                Uri uri = Uri.parse("android.resource://" + getPackageName() + "/" + R.raw.elton_john);
                try {
                    play.setVisibility(View.INVISIBLE);
                    stop.setVisibility(View.VISIBLE);
                    mediaPlayer.setDataSource(ActivityMediaPlayer.this,uri);
                    mediaPlayer.prepare();
                    mediaPlayer.start();
                    Toast.makeText(ActivityMediaPlayer.this, "Comienza reproducción ", Toast.LENGTH_LONG).show();
                } catch (IOException e){
                    Toast.makeText(ActivityMediaPlayer.this, "Error al reproducir", Toast.LENGTH_LONG).show();
                }
            }
        });

        stop.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View view) {
                if(mediaPlayer != null && mediaPlayer.isPlaying()){
                    play.setVisibility(View.VISIBLE);
                    stop.setVisibility(View.INVISIBLE);
                    mediaPlayer.stop();
                    mediaPlayer = null;
                    Toast.makeText(ActivityMediaPlayer.this, "Se detiene reproducción ", Toast.LENGTH_LONG).show();
                }
            }
        });
    }
}