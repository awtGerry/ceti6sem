package com.example.diagnostico;

import androidx.appcompat.app.AppCompatActivity;

import android.content.Context;
import android.content.Intent;
import android.content.SharedPreferences;
import android.os.Bundle;
import android.view.View;
import android.widget.EditText;
import android.widget.RadioButton;
import android.widget.Toast;

public class RegisterActivity extends AppCompatActivity {

    EditText name;
    RadioButton male, female, other;

    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_register);

        name = (EditText) findViewById(R.id.id_name);
        male = (RadioButton) findViewById(R.id.id_male);
        female = (RadioButton) findViewById(R.id.id_female);
        other = (RadioButton) findViewById(R.id.id_other);
    }

    public void register(View view) {
        String selection;
        if (male.isSelected()) {
            selection = "Hombre";
        } else if (female.isSelected()) {
            selection = "Mujer";
        } else {
            selection = "Otro";
        }
        SharedPreferences data = getSharedPreferences("data", MODE_PRIVATE);
        SharedPreferences.Editor editor = data.edit();
        editor.putString("data.name", name.getText().toString());
        editor.putString("data.gender", selection);
        editor.commit();
        Toast.makeText(this, "Datos registrados.", Toast.LENGTH_SHORT).show();
    }

    public void go_back(View view) {
        Intent intent = new Intent(this, MenuActivity.class);
        startActivity(intent);
    }
}