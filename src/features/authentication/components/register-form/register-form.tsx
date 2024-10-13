import { UseFormReturn } from "react-hook-form";
import { UseRegisterQueryParams } from "../../api/use-register-query";

export type RegisterFormProps = {
  form: UseFormReturn<UseRegisterQueryParams, unknown>;
  onSubmit: (data: UseRegisterQueryParams) => void;
};

export function RegisterForm({
  form: { handleSubmit, register },
  onSubmit,
}: RegisterFormProps) {
  return (
    <form onSubmit={handleSubmit(onSubmit)}>
      <input {...register("email")} />
      <input {...register("username")} />
      <input {...register("password")} />
      <input {...register("password_confirmation")} />

      <button type="submit">Envoyer</button>
    </form>
  );
}
