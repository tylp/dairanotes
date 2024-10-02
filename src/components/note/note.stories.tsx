import type { Meta, StoryObj } from "@storybook/react";
import { Note } from "./note";

const meta = {
  component: Note,
  tags: ["autodocs"],
  args: {
    children: "Coucou tout le monde",
  },
  argTypes: {},
} satisfies Meta<typeof Note>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {},
};
