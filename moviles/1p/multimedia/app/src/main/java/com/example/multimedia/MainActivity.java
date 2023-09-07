package com.example.multimedia;

import androidx.appcompat.app.AppCompatActivity;

import android.content.Intent;
import android.os.Bundle;
import android.view.View;

public class MainActivity extends AppCompatActivity {

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
    }

    public void onClick(View v){
        Intent intent = null;
        switch (v.getId()){
            case R.id.btnMusic:
                intent = new Intent(this, ActivityMusicPlayer.class);
                break;
            case R.id.btnMedia:
                intent = new Intent(this, ActivityMediaPlayer.class);
                break;
            case R.id.btnVideo:
                intent = new Intent(this, ActivityVideoView.class);
                break;
        }
        startActivity(intent);
    }
}