import { BlockSpec } from '@blocksuite/block-std';
import { Doc } from '@blocksuite/store';
import { useState } from 'react';

export default function EditorHost() {
  // BlockSuite architects its editors as assemblies of multiple editable blocks, termed BlockSpecs.
  // Each block spec encapsulates the data schema, view, service, and logic required to compose the editor.
  const [specs, setSpecs] = useState<BlockSpec[]>();
  // In BlockSuite, each doc object manages an independent block tree composed of various types of blocks.
  // These blocks can be defined through the BlockSchema, which specifies their fields and permissible nesting relationships among different block types.
  const [doc, setDoc] = useState<Doc>();

  return <div>Hello World 🍀</div>;
}
