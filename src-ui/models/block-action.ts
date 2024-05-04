export class BlockAction {
  action_type: string;
  index: number;
  from_index: number;
  to_index: number;
  block: {
    id: string;
    tool: string;
    type: string;
    data: any;
  };
}
