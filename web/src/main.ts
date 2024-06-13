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

const editorView = new EditorView({
  parent: document.getElementById("editor")!,
  state: EditorState.create({
    doc: 'println("hello, world");',
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

const examples: Record<string, string> = {
  "Variable Declaration": `x := 10;
println("the value of x is", x);`,

  "Variable Assignment": `x := 10;
println("the value of x is", x);

x = 5;
println("the value of x is", x);`,

  "If-else": `if (true) {
  println("Hello, World");
} else {
  println("dlroW, olleH");
};`,

  "Functions": `add := fn(x, y) {
  return x + y;
};

println("1 + 2 =", add(1, 2));`,

  "Closures": `adder := fn() {
  sum := 0;
  return fn(n) {
    sum = sum + n;
    return sum;
  };
};

f := adder();
g := adder();

x := 1;
while (x < 10) {
  println("f =", f(1), "| g =", g(2));
  x = x + 1;
}`,

  "Factorial": `fact := fn(n) {
  if (n < 2) {
    return 1;
  } else {
    return n * fact(n - 1);
  };
};

println(fact(5));`
}

const exampleSelectorEl = document.getElementById("example-selector")! as HTMLSelectElement;

for (const key in examples) {
  const option = document.createElement("option");
  option.value = key;
  option.append(key);

  exampleSelectorEl.appendChild(option);
}
