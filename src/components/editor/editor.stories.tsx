import type { Meta, StoryObj } from "@storybook/react";
import { Editor } from "./editor";

const meta = {
  component: Editor,
  tags: ["autodocs"],
  render: () => <Editor />,
} satisfies Meta<typeof Editor>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {},
};
