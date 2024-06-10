import './main.css'
import * as rs from "rs";

import { closeBrackets, closeBracketsKeymap } from '@codemirror/autocomplete';
import { defaultKeymap, history, historyKeymap, indentWithTab } from '@codemirror/commands';
import { bracketMatching, indentOnInput } from '@codemirror/language';
import { EditorView, highlightActiveLine, highlightActiveLineGutter, keymap, lineNumbers } from '@codemirror/view';
import { EditorState } from '@codemirror/state';

console.log(rs);

function runCode(view: EditorView) {
  rs.run_code(view.state.doc.toString());
  return true;
}

new EditorView({
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


