package com.example.diagnostico;

import androidx.appcompat.app.AppCompatActivity;

import android.content.Intent;
import android.os.Bundle;
import android.view.View;
import android.widget.EditText;
import android.widget.Toast;

public class MainActivity extends AppCompatActivity {

    private EditText username, password;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
        username = (EditText) findViewById(R.id.id_uname);
        password = (EditText) findViewById(R.id.id_passwd);
    }
    
    public void login(View view) {
        if (!username.getText().toString().isEmpty()) {
            Intent intent = new Intent(this, MenuActivity.class);
            startActivity(intent);
        } else {
            Toast.makeText(this, "Nombre de usuario o contrasena incorrectos", Toast.LENGTH_SHORT).show();
        }
    }
}