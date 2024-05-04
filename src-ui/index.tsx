import './editor.css';

import Checklist from '@editorjs/checklist';
import { API, BlockMutationEvent, ToolConstructable } from '@editorjs/editorjs';
import { SavedData } from '@editorjs/editorjs/types/data-formats';
import { invoke } from '@tauri-apps/api/tauri';
import * as React from 'react';
import { useState } from 'react';
import * as ReactDOM from 'react-dom/client';

import Editor from './components/Editor';
import { BlockData } from './models';
import { BlockAction } from './models/block-action';

function App() {
  const tauri = window.__TAURI__;

  let is_first = true;

  const [blocks, setBlocks] = useState([]);

  const transformEventModel = async (api: API, e: BlockMutationEvent) => {
    let savedData: BlockData<any> = { data: {} };
    if (e.type !== 'block-removed') {
      const block = api.blocks.getById(e.detail.target.id);
      savedData = (await block.save()) as SavedData;
    }
    return {
      action_type: e.type,
      index: (e.detail as any).index,
      from_index: (e.detail as any).fromIndex,
      to_index: (e.detail as any).toIndex,
      block: {
        id: savedData.id,
        tool: savedData.tool,
        data: savedData && savedData.data,
      },
    };
  };

  if (typeof tauri !== 'undefined') {
    invoke('history').then((docs: BlockAction[]) => {
      const history = [];
      if (docs && docs.length) {
        is_first = false;
        for (const doc of docs) {
          doc.block.type = doc.block.tool;
          delete doc.block.tool;
          history.push(doc.block);
        }
      } else {
        history.push({
          id: '_',
          type: 'paragraph',
          data: { text: 'HELLO SCRAPBOOK!' },
        });
      }
      setBlocks([...history]);
    });
  }

  const onReady = async () => {
    if (is_first && typeof tauri !== 'undefined') {
      invoke('document_change', {
        events: [
          {
            action_type: 'block-added',
            index: 0,
            block: blocks[0],
          },
        ],
      });
    }
  };

  const onChange = async (
    api: API,
    event: BlockMutationEvent | BlockMutationEvent[],
  ) => {
    const events = [];
    if (Array.isArray(event)) {
      for (let i = 0; i < event.length; i++) {
        events.push(await transformEventModel(api, event[i]));
      }
    } else {
      events.push(await transformEventModel(api, event));
    }
    if (typeof tauri !== 'undefined') {
      invoke('document_change', { events: events });
    }
  };

  return (
    <div>
      <h1>Scrapbook</h1>
      <Editor
        tools={{
          checklist: {
            class: Checklist as ToolConstructable,
            inlineToolbar: true,
          },
        }}
        onReady={onReady}
        onChange={onChange}
        data={{
          blocks: blocks as any,
        }}
      ></Editor>
    </div>
  );
}

ReactDOM.createRoot(document.getElementById('app')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
