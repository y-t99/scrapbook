import {
  DividerBlockSchema,
  ListBlockSchema,
  NoteBlockSchema,
  ParagraphBlockSchema,
  RootBlockSchema,
} from '@blocksuite/blocks';
import { BlockSchema } from '@blocksuite/store';
import { z } from 'zod';

// In BlockSuite, all blocks should have a schema. The schema of the block describes the data structure of the block.
export const ScrapbookSchemas: z.infer<typeof BlockSchema>[] = [
  ParagraphBlockSchema,
  NoteBlockSchema,
  RootBlockSchema,
  ListBlockSchema,
  DividerBlockSchema,
];
