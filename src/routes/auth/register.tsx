import {
  useRegisterQuery,
  UseRegisterQueryParams,
  UseRegisterQueryParamsSchema,
} from "@/features/authentication/api/use-register-query";
import { RegisterForm } from "@/features/authentication/components/register-form";
import { useAuthStore } from "@/features/authentication/stores/use-auth-store";
import { zodResolver } from "@hookform/resolvers/zod";
import {
  createFileRoute,
  Link,
  redirect,
  useNavigate,
} from "@tanstack/react-router";
import { useForm } from "react-hook-form";

export const Route = createFileRoute("/auth/register")({
  component: Register,
  beforeLoad: () => {
    const { isAuthenticated } = useAuthStore.getState();

    if (isAuthenticated()) {
      throw redirect({
        to: "/notes",
        search: {
          redirect: location.href,
        },
      });
    }
  },
});

function Register() {
  const form = useForm<UseRegisterQueryParams>({
    resolver: zodResolver(UseRegisterQueryParamsSchema),
  });

  const navigate = useNavigate();

  const { login } = useAuthStore();

  const { mutateAsync: register } = useRegisterQuery();

  const onSubmit = async (data: UseRegisterQueryParams) => {
    const registeredUser = await register(data);

    form.reset();
    login(registeredUser);
    navigate({
      to: "/notes",
      search: {
        redirect: location.href,
      },
    });
  };

  return (
    <div className="flex flex-col gap-2 p-2">
      <h3>Page d'inscription</h3>

      <RegisterForm form={form} onSubmit={onSubmit} />

      <Link to="/auth/login">Déjà un compte?</Link>
    </div>
  );
}
