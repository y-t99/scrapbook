import EditorJS, { OutputData } from '@editorjs/editorjs';

export interface EditorCore {
  /**
   * @see Saver.save
   */
  save(): Promise<OutputData>;

  /**
   * @see Blocks.clear
   */
  clear(): void;

  /**
   * @see Blocks.render
   */
  render(data: OutputData): Promise<void>;

  /**
   * Destroy Editor instance and related DOM elements
   */
  destroy(): void;

  get dangerouslyLowLevelInstance(): EditorJS | null;
}
