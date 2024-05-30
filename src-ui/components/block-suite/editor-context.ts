import { DocCollection } from '@blocksuite/store';
import { createContext, JSX, useContext } from 'react';

export const EditorContext = createContext<{
  editor: JSX.Element;
  collection: DocCollection;
} | null>(null);

export function useEditor() {
  return useContext(EditorContext);
}

export interface EditorContextType {
  editor: JSX.Element | null;
  collection: DocCollection | null;
  updateCollection: (newCollection: DocCollection) => void;
}
