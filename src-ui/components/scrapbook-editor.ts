import EditorJS, { EditorConfig, OutputData } from '@editorjs/editorjs';

import { EditorCore } from './editor-core';

export class ScrapbookEditor implements EditorCore {
  private readonly _editorJS: EditorJS;

  constructor({ tools, ...config }: EditorConfig) {
    const extendTools = {
      ...tools,
    };

    this._editorJS = new EditorJS({
      tools: extendTools,
      ...config,
    });
  }

  public get dangerouslyLowLevelInstance() {
    return this._editorJS;
  }

  public async clear() {
    this._editorJS.clear();
  }

  public async save() {
    return this._editorJS.save();
  }

  public async destroy() {
    await this._editorJS.isReady;
    this._editorJS.destroy();
  }

  public async render(data: OutputData) {
    await this._editorJS.render(data);
  }
}
