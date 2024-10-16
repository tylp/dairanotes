import z from "zod";
import { User } from "@/types/user";
import { useMutation } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";

export const UseRegisterQueryParamsSchema = z
  .object({
    email: z.string().email(),
    username: z.string().min(3).max(20),
    password: z.string().min(8),
    password_confirmation: z.string().min(8),
  })
  .refine((data) => data.password === data.password_confirmation, {
    path: ["password_confirmation"],
  });

export type UseRegisterQueryParams = z.infer<
  typeof UseRegisterQueryParamsSchema
>;

export type UseRegisterQueryReturn = User;

const mutationFn = (
  params: UseRegisterQueryParams,
): Promise<UseRegisterQueryReturn> => {
  return invoke("register", params);
};

export const useRegisterQuery = () => {
  return useMutation({
    mutationFn,
  });
};
