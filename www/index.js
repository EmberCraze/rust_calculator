import * as wasm from "calculator_wasm";

document.getElementById("button_eq").addEventListener("click", button_eq_input);

function modify_output_textfield(input,) {
    var output_text_field = document.getElementById("output_text_field");
    output_text_field.value = input;
}
function button_eq_input() {
    var input_text_field = document.getElementById("input_text_field");
    
    //wasm.greet("Maher");
    var result = wasm.parse_input(input_text_field.value);
    modify_output_textfield(result);
}

document.getElementById("input_text_field").addEventListener("keypress", (event) => {
    var name = event.key;
    var code = event.code;
    if (name === "Enter"){
        var result = wasm.parse_input(input_text_field.value);
        modify_output_textfield(result);
    }
})

//wasm.greet("Maher");
