import { EditorHost } from '@blocksuite/block-std';
import { createComponent } from '@lit/react';
import React from 'react';

const ReactHost = createComponent({
  tagName: 'editor-host',
  react: React,
  elementClass: EditorHost,
});

export default ReactHost;
