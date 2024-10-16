import { UseFormReturn } from "react-hook-form";
import { UseRegisterQueryParams } from "../../api/use-register-query";
import { TextInput } from "@/components/text-input";
import { Form } from "@/components/form";

export type RegisterFormProps = {
  form: UseFormReturn<UseRegisterQueryParams, unknown>;
  onSubmit: (data: UseRegisterQueryParams) => void;
};

export function RegisterForm({
  form: { handleSubmit, register },
  onSubmit,
}: RegisterFormProps) {
  return (
    <Form onSubmit={handleSubmit(onSubmit)}>
      <Form.Field name="email">
        <Form.Field.Label>Email</Form.Field.Label>
        <Form.Field.Control asChild>
          <TextInput {...register("email")} />
        </Form.Field.Control>
        <Form.Message />
      </Form.Field>

      <Form.Field name="username">
        <Form.Field.Label>Nom d'utilisateur</Form.Field.Label>
        <Form.Field.Control type="password" asChild>
          <TextInput {...register("username")} type="password" />
        </Form.Field.Control>
        <Form.Message />
      </Form.Field>

      <Form.Field name="password">
        <Form.Field.Label>Mot de passe</Form.Field.Label>
        <Form.Field.Control type="password" asChild>
          <TextInput {...register("password")} type="password" />
        </Form.Field.Control>
        <Form.Message />
      </Form.Field>

      <Form.Field name="password_confirmation">
        <Form.Field.Label>Confirmer le mot de passe</Form.Field.Label>
        <Form.Field.Control type="password" asChild>
          <TextInput {...register("password_confirmation")} type="password" />
        </Form.Field.Control>
        <Form.Message />
      </Form.Field>

      <button type="submit">Envoyer</button>
    </Form>
  );
}
