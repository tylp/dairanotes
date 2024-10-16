import type { Meta, StoryObj } from "@storybook/react";
import { Paper } from "./paper";

const meta = {
  component: Paper,
  tags: ["autodocs"],
  args: {
    children: "Coucou tout le monde",
  },
  argTypes: {},
} satisfies Meta<typeof Paper>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {},
};
