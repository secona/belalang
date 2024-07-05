import './main.css'
import * as rs from "rs";

import { closeBrackets, closeBracketsKeymap } from '@codemirror/autocomplete';
import { defaultKeymap, history, historyKeymap, indentWithTab } from '@codemirror/commands';
import { bracketMatching, indentOnInput } from '@codemirror/language';
import { EditorView, highlightActiveLine, highlightActiveLineGutter, keymap, lineNumbers } from '@codemirror/view';
import { EditorState } from '@codemirror/state';

function clamp(min: number, value: number, max: number): number {
  return Math.max(min, Math.min(value, max));
}

const handle = document.getElementById("handle")!;
const editor = document.getElementById("editor")!;
const out = document.getElementById("out-wrapper")!;

let dragging = false;

handle.addEventListener("mousedown", _ => {
  dragging = true;
});

document.addEventListener("mousemove", e => {
  if (dragging) {
    const editorWidth = clamp(20, (e.clientX / window.innerWidth) * 100, 80);
    const outWidth = 100 - editorWidth;

    editor.style.width = `${editorWidth}%`;
    out.style.width = `${outWidth}%`;
  }
});

document.addEventListener("mouseup", _ => {
  dragging = false;
})

function runCode() {
  rs.run_code(editorView.state.doc.toString());
  return true;
}
(window as any).runCode = runCode;

function clearConsole() {
  const out = document.getElementById("out")!;

  while (out.firstChild) {
    out.removeChild(out.lastChild!);
  }
}
(window as any).clearConsole = clearConsole;

function println(value: string) {
  let parent = document.getElementById("out")!;

  let p = document.createElement("p");
  p.textContent = value;

  parent.appendChild(p);
}
(window as any).println = println;

function exampleSelector() {
  const selector = document.getElementById("example-selector")! as HTMLSelectElement;

  editorView.dispatch(editorView.state.update({
    changes: {
      from: 0,
      to: editorView.state.doc.length,
      insert: examples[selector.value],
    }
  }));

  editorView.focus();

  selector.selectedIndex = 0;
}
(window as any).exampleSelector = exampleSelector;

const examples: Record<string, string> = {
  "Hello World": `println("Hello, World!");`,

  "Variables": `# Declare variables using the \`:=\` operator
# Currently, Belalang only supports integers, floats, and strings
int_var := 123;
flo_var := 44.2;
str_var := "Hello, World!";

# Print variables to the console using the builtin \`println\` function
println("the value of int_var is", int_var);
println("the value of flo_var is", flo_var);
println("the value of str_var is", str_var);

# Assign new values to existing variables using the \`=\` operator
int_var = 1;
flo_var = 1.4;
str_var = "Hello, Mom!";

# Print the new values of the variables
println("the new value of int_var is", int_var);
println("the new value of flo_var is", flo_var);
println("the new value of str_var is", str_var);`,

  "If-Else If-Else": `# Variable declaration and assignment
price := 50;

# Conditional statement example
if (price < 30) {
    println("very cheap");
} else if (price < 40) {
    println("not so cheap");
} else {
    println("very not cheap");
}

# Using if as an expression
status := if (price < 30) {
    "very cheap"
} else if (price < 40) {
    "not so cheap"
} else {
    "very not cheap"
};

# Print the status
println(status);`,

  "Functions": `# Functions are declared like variables using the \`fn\` keyword
# Define a function to add two numbers
add := fn(x, y) {
    return x + y;
};

# Call the add function and print the result
println("1 + 2 =", add(1, 2));

# You can also take advantage of expressions to return values
# Define a function to multiply two numbers using an expression
mul := fn(x, y) { x * y };

# Call the mul function and print the result
println("3 * 4 =", mul(3, 4));`,

  "While Loops": `# While loops are declared with the \`while\` keyword.
i := 0;
while (i < 10) {
  println("i =", i);
  i = i + 1;
}`,

  "Closures": `# Closures!
adder := fn() {
  sum := 0;
  return fn(n) {
    sum = sum + n;
    return sum;
  };
};

# Each instance of adder has different
# values of sum
f := adder();
g := adder();

x := 1;
while (x < 10) {
  println("f =", f(1), "| g =", g(2));
  x = x + 1;
}`,

  "Factorial": `# Recursions!
fact := fn(n) {
  # The base case
  if (n < 2) {
    return 1;
  }
  
  # The recursive case
  return n * fact(n - 1);
};

println("5! =", fact(5));`
}

const exampleSelectorEl = document.getElementById("example-selector")! as HTMLSelectElement;

for (const key in examples) {
  const option = document.createElement("option");
  option.value = key;
  option.append(key);

  exampleSelectorEl.appendChild(option);
}

const editorView = new EditorView({
  parent: document.getElementById("editor")!,
  state: EditorState.create({
    doc: examples["Hello World"],
    extensions: [
      bracketMatching(),
      closeBrackets(),
      history(),
      highlightActiveLine(),
      highlightActiveLineGutter(),
      indentOnInput(),
      lineNumbers(),
      keymap.of([
        { key: "Mod-Enter", run: runCode },
        indentWithTab,
        ...defaultKeymap,
        ...historyKeymap,
        ...closeBracketsKeymap,
      ]),
    ],
  }),
});
