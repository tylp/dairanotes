import * as RadixForm from "@radix-ui/react-form";
import { twMerge } from "tailwind-merge";

export function Message({ className, ...props }: RadixForm.FormMessageProps) {
  return (
    <RadixForm.Message
      {...props}
      className={twMerge("text-right text-sm", className)}
    />
  );
}
