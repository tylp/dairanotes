import { PropsWithChildren } from "react";
import { paperStyles } from "./styles";
import { Color, Padding } from "./types";

export type PaperProps = PropsWithChildren & {
  color?: Color;
  padding?: Padding;
};

export function Paper({ color = "red", padding = "md", children }: PaperProps) {
  return (
    <article className={paperStyles({ color, padding })}>{children}</article>
  );
}
