import EditorJS from '@editorjs/editorjs';
import Checklist from '@editorjs/checklist';
import './editor.css';

const tauri = window.__TAURI__;

let is_first = true;

let blocks = [];

if (typeof tauri !== "undefined") {
    const invoke = tauri.invoke;

    const transformEventModel = async (api, e) => {
        let savedData = {data: {}};
        if (e.type !== 'block-removed') {
            const block = api.blocks.getById(e.detail.target.id);
            savedData = await block.save();
        }
        return {
            action_type: e.type,
            index: e.detail.index,
            from_index: e.detail.fromIndex,
            to_index: e.detail.toIndex,
            block: {
                id: savedData.id,
                tool: savedData.tool,
                data: savedData && savedData.data
            }
        }
    };

    invoke('history').then((docs) => {
        if (docs && docs.length) {
            is_first = false;
            for (const doc of docs) {
                doc.block.type = doc.block.tool;
                delete doc.block.tool;
                blocks.push(doc.block);
            }
        } else {
            blocks.push({
                id: "_",
                type: "paragraph",
                data: {text: "HELLO SCRAPBOOK!"}
            });
        }
        const editor = new EditorJS({
            holder: 'scrapbook_page',

            tools: {
                checklist: {
                    class: Checklist,
                    inlineToolbar: true
                }
            },

            onReady: async () => {
                const invoke = tauri.invoke;
                if (is_first) {
                    invoke('document_change', {
                        events: [
                            {
                                action_type: 'block-added',
                                index: 0,
                                block: blocks[0],
                            }
                        ]
                    });
                }
            },

            onChange: async (api, event) => {
                if (typeof tauri !== "undefined") {
                    const invoke = tauri.invoke;
                    const events = [];
                    if (Array.isArray(event)) {
                        for (let i = 0; i < event.length; i++) {
                            events.push(await transformEventModel(api, event[i]));
                        }
                    } else {
                        events.push(await transformEventModel(api, event));
                    }
                    invoke('document_change', {events: events});
                }
            },

            data: {
                blocks: blocks
            }
        });
    });
}