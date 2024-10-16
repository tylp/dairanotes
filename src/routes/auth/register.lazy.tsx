import {
  useRegisterQuery,
  UseRegisterQueryParams,
  UseRegisterQueryParamsSchema,
} from "@/features/authentication/api/use-register-query";
import { RegisterForm } from "@/features/authentication/components/register-form";
import { useAuthStore } from "@/features/authentication/stores/use-auth";
import { zodResolver } from "@hookform/resolvers/zod";
import { createLazyFileRoute, Link } from "@tanstack/react-router";
import { useForm } from "react-hook-form";

export const Route = createLazyFileRoute("/auth/register")({
  component: Register,
});

function Register() {
  const form = useForm<UseRegisterQueryParams>({
    resolver: zodResolver(UseRegisterQueryParamsSchema),
  });

  const { login } = useAuthStore();

  const { mutateAsync: register } = useRegisterQuery();

  const onSubmit = async (data: UseRegisterQueryParams) => {
    const user = await register(data);

    login(user);
  };

  return (
    <div className="flex flex-col gap-2 p-2">
      <h3>Page d'inscription</h3>

      <RegisterForm form={form} onSubmit={onSubmit} />

      <Link to="/auth/login">Déjà un compte?</Link>
    </div>
  );
}
