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


