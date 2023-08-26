package com.example.diagnostico;

import androidx.appcompat.app.AppCompatActivity;

import android.content.SharedPreferences;
import android.os.Bundle;
import android.widget.TextView;

public class ShowActivity extends AppCompatActivity {

    TextView results;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_show);
        results = (TextView) findViewById(R.id.id_results);
        SharedPreferences preferences = getSharedPreferences("data", MODE_PRIVATE);
        SharedPreferences.Editor editor = preferences.edit();

        String name = preferences.getString("data.name", "");
        String gender = preferences.getString("data.gender", "");
        results.setText(name + "\n" + gender);
    }
}