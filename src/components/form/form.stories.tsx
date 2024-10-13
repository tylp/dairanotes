import { Meta, StoryObj } from "@storybook/react";
import { Form } from "./form";

const meta = {
  component: Form,
  tags: ["autodocs"],
  parameters: {
    layout: "centered",
  },
  render: () => (
    <Form className="w-60">
      <Form.Field name="email">
        <Form.Field.Label>Email</Form.Field.Label>
        <Form.Field.Control />
        <Form.Message />
      </Form.Field>

      <Form.Field name="password">
        <Form.Field.Label>Mot de passe</Form.Field.Label>
        <Form.Field.Control type="password" />
        <Form.Message />
      </Form.Field>

      <Form.Field name="confirmPassword">
        <Form.Field.Label>Confirmer le mot de passe</Form.Field.Label>
        <Form.Field.Control type="password" />
        <Form.Message />
      </Form.Field>

      <Form.Field name="terms">
        <Form.Field.Label>Accepter les conditions générales</Form.Field.Label>
        <Form.Field.Control type="checkbox" />
        <Form.Message />
      </Form.Field>

      <Form.Field name="submit">
        <Form.Field.Label>Soumettre</Form.Field.Label>
        <Form.Field.Control type="submit" />
        <Form.Message />
      </Form.Field>
    </Form>
  ),
  argTypes: {},
} satisfies Meta<typeof Form>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {},
};
