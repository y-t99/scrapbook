import { EditorConfig } from '@editorjs/editorjs';
import * as React from 'react';

import { EditorCore } from './editor-core';
import { ScrapbookEditor } from './scrapbook-editor';

export default function Editor(editorConfig: EditorConfig) {
  const memoizedHolder = React.useRef('scrapbook_page');

  const editor = React.useRef<EditorCore | null>(null);
  editor.current = new ScrapbookEditor({
    holder: memoizedHolder.current,
    ...editorConfig,
  });

  return (
    <form action="">
      <div id={memoizedHolder.current}></div>
    </form>
  );
}
