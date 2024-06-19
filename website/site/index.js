import * as rust from "website";

function makeOutput() {
  var input = document.getElementById('inputBox').value;
  var output = document.getElementById('output');
  output.innerHTML = rust.process_input(input);
}

document.getElementById('inputBox').value = rust.get_example();
window.makeOutput = makeOutput

