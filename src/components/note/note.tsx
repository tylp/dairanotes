import { PropsWithChildren } from "react";
import { noteStyles } from "./styles";
import { Color, Padding } from "./types";

export type NoteProps = PropsWithChildren & {
  color?: Color;
  padding?: Padding;
};

export function Note({ color = "red", padding = "md", children }: NoteProps) {
  return (
    <article className={noteStyles({ color, padding })}>{children}</article>
  );
}
