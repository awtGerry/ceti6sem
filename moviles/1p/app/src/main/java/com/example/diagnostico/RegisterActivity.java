package com.example.diagnostico;

import androidx.appcompat.app.AppCompatActivity;

import android.os.Bundle;
import android.view.View;
import android.widget.EditText;
import android.widget.RadioButton;

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
        Person person = new Person();
        String selection;
        if (male.isSelected()) {
            selection = "Hombre";
        } else if (female.isSelected()) {
            selection = "Mujer";
        } else {
            selection = "Otro";
        }
        person.setName(name.getText().toString());
        person.setGender(selection);
    }
}