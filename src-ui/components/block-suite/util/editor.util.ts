import '../themes/themes.css';

import { DocCollection, Schema } from '@blocksuite/store';

import EditorContainer from '../EditorContainer';
import { ScrapbookSchemas } from '../schema';

export function initEditor(docId = 'scrapbook') {
  const schema = new Schema().register(ScrapbookSchemas);
  const collection = new DocCollection({ schema });
  const doc = collection.createDoc({ id: docId });

  doc.load(() => {
    const pageBlockId = doc.addBlock('affine:page', {});
    const noteId = doc.addBlock('affine:note', {}, pageBlockId);
    doc.addBlock('affine:paragraph', {}, noteId);
  });

  const editor = EditorContainer();
  // editor.doc = doc;
  // editor.slots.docLinkClicked.on(({ docId }) => {
  //   editor.doc = <Doc>collection.getDoc(docId);
  // });

  return { editor, collection };
}
