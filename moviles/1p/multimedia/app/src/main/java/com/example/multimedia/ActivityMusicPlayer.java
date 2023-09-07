package com.example.multimedia;

import androidx.appcompat.app.AppCompatActivity;

import android.annotation.SuppressLint;
import android.media.MediaPlayer;
import android.os.Bundle;
import android.os.Handler;
import android.view.View;
import android.widget.ImageButton;
import android.widget.SeekBar;
import android.widget.TextView;

import java.util.Locale;
import java.util.concurrent.TimeUnit;

public class ActivityMusicPlayer extends AppCompatActivity {

    private ImageButton play, pause, forward, backward;
    private TextView song, duration, current, song_artist;
    int finalTime;
    private SeekBar seekBar;
    final int TIME = 5000;
    int currentPosition = 0;
    private MediaPlayer mediaPlayer;
    private Handler handler;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_music_player);

        play = findViewById(R.id.musicPlay);
        pause = findViewById(R.id.musicPause);
        forward = findViewById(R.id.musicForward);
        backward = findViewById(R.id.musicBack);
        song = findViewById(R.id.musicArtist);
        song_artist = findViewById(R.id.musicSong);
        duration = findViewById(R.id.musicDuration);
        current = findViewById(R.id.musicTime);
        seekBar = findViewById(R.id.musicSeekBar);
        mediaPlayer = MediaPlayer.create(this,R.raw.elton_john);
        handler = new Handler();
        pause.setEnabled(false);

        play.setOnClickListener(new View.OnClickListener() {
            @SuppressLint("SetTextI18n")
            @Override
            public void onClick(View view) {
                play.setVisibility(View.INVISIBLE);
                pause.setVisibility(View.VISIBLE);
                mediaPlayer.start();
                song.setText("Sacrifice");
                song_artist.setText("Elton John");
                finalTime = mediaPlayer.getDuration();
                seekBar.setMax(finalTime);
                duration.setText(String.format(Locale.getDefault(),
                        "%d:%d",
                        TimeUnit.MILLISECONDS.toMinutes((long) finalTime),
                        TimeUnit.MILLISECONDS.toSeconds((long) finalTime)
                                % 60
                ));
                handler.postDelayed(UpdateTime, 100);
                pause.setEnabled(true);
                play.setEnabled(false);
            }
        });

        pause.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View view) {
                play.setVisibility(View.VISIBLE);
                pause.setVisibility(View.INVISIBLE);
                mediaPlayer.pause();
                pause.setEnabled(false);
                play.setEnabled(true);
            }
        });

        forward.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View view) {
                if(currentPosition + TIME <= finalTime){
                    mediaPlayer.seekTo((int)(currentPosition + TIME));
                } else {
                    mediaPlayer.seekTo(finalTime);
                }
            }
        });

        backward.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View view) {
                if(currentPosition - TIME >= 0){
                    mediaPlayer.seekTo((int)(currentPosition - TIME));
                } else {
                    mediaPlayer.seekTo(0);
                }
            }
        });
    }

    private Runnable UpdateTime = new Runnable() {
        @Override
        public void run() {
            currentPosition = mediaPlayer.getCurrentPosition();
            current.setText(String.format(Locale.getDefault(),
                    "%d:%d",
                    TimeUnit.MILLISECONDS.toMinutes((long)currentPosition),
                    TimeUnit.MILLISECONDS.toSeconds((long)currentPosition) % 60
            ));
            seekBar.setProgress((int) currentPosition);
            handler.postDelayed(this, 100);
        }
    };
}