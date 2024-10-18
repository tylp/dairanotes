import { Meta, StoryObj } from "@storybook/react";
import { TextInput } from "./text-input";

const meta = {
  component: TextInput,
  tags: ["autodocs"],
  parameters: {
    layout: "centered",
  },
  argTypes: {},
} satisfies Meta<typeof TextInput>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {},
};

export const Icons: Story = {
  args: {},
};
