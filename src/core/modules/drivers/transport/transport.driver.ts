import { PostDto } from '../dto/post.dto';
export type PostOpts = {
  id?: string;
  url?: string;
  username?: string;
  password?: string;
  group_id?: string;
  status?: string;
};

export abstract class TransportDriver {
  abstract post(data: PostDto, opts?: PostOpts): Promise<void>;
}
