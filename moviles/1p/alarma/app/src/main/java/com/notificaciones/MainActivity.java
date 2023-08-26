package com.notificaciones;

import androidx.appcompat.app.AppCompatActivity;

import android.content.Intent;
import android.os.Bundle;
import android.view.View;
import android.widget.EditText;
import android.widget.Toast;

public class MainActivity extends AppCompatActivity {

    private EditText uname, passwd;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
        uname = (EditText) findViewById(R.id.id_uname);
        passwd = (EditText) findViewById(R.id.id_passwd);
    }

    public void login(View view) {
        String username = uname.getText().toString();
        String password = passwd.getText().toString();
        if (username.isEmpty() || password.isEmpty()) {
            Toast.makeText(this, "Faltan campos por rellenar", Toast.LENGTH_SHORT).show();
        } else {
            if (username.equals("hospital") && password.equals("admin")) {
                Intent dashboard = new Intent(this, DashboardActivity.class);
                startActivity(dashboard);
            } else {
                Toast.makeText(this, "Usuario o contrasena incorrectos", Toast.LENGTH_SHORT).show();
            }
        }
    }
}