//import * as wasm from "calculator_wasm";
window.gwd = window.gwd || {};
    gwd.hide_buttons = function(event) {
      var x = document.getElementById("input_buttons2");
      if (x.style.display === "none") {
        x.style.display = "block";
      } else {
        x.style.display = "none";
      }
    };
    gwd.press1 = function(event) {
      var text = document.getElementById('input_text_field');
      text.value += '1';
    };
    gwd.press2 = function(event) {
      var text = document.getElementById('input_text_field');
      text.value += '2';
    };
    gwd.press3 = function(event) {
      var text = document.getElementById('input_text_field');
      text.value += '3';
    };
    gwd.press4 = function(event) {
      var text = document.getElementById('input_text_field');
      text.value += '4';
    };
    gwd.press6 = function(event) {
      var text = document.getElementById('input_text_field');
      text.value += '6';
    };
    gwd.press5 = function(event) {
      var text = document.getElementById('input_text_field');
      text.value += '5';
    };
    gwd.press7 = function(event) {
      var text = document.getElementById('input_text_field');
      text.value += '7';
    };
    gwd.press8 = function(event) {
      var text = document.getElementById('input_text_field');
      text.value += '8';
    };
    gwd.press9 = function(event) {
      var text = document.getElementById('input_text_field');
      text.value += '9';
    };
    gwd.press0 = function(event) {
      var text = document.getElementById('input_text_field');
      text.value += '0';
    };
    gwd.press_dot = function(event) {
      var text = document.getElementById('input_text_field');
      text.value += '.';
    };
    gwd.press_eq = function(event) {
      //let wasm = await import("calculator_wasm");
      //var text = document.getElementById('input_text_field');
      //text.value += '=';
      //import * as wasm from "calculator_wasm";

      wasm.greet("Maher");
    };
    gwd.press_div = function(event) {
      var text = document.getElementById('input_text_field');
      text.value += '/';
    };
    gwd.press_mult = function(event) {
      var text = document.getElementById('input_text_field');
      text.value += '*';
    };
    gwd.press_plus = function(event) {
      var text = document.getElementById('input_text_field');
      text.value += '+';
    };
    gwd.press_minus = function(event) {
      var text = document.getElementById('input_text_field');
      text.value += '-';
    };
    gwd.remove_last_caracter = function(event) {
      var text = document.getElementById('input_text_field');
      var modified_text = text.value.slice(0, -1);
      text.value = modified_text;
    };