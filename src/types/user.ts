import { z } from "zod";

export const UserSchema = z.object({
  id: z.string().uuid(),
  name: z.string().min(1).max(100),
  email: z.string().email(),
  created_at: z.date(),
});

export type User = z.infer<typeof UserSchema>;
