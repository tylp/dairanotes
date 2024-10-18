import * as RadixForm from "@radix-ui/react-form";
import { Field } from "./field";
import { Message } from "./message";
import { twMerge } from "tailwind-merge";

export function Form({ className, ...props }: RadixForm.FormProps) {
  return (
    <RadixForm.Root
      {...props}
      className={twMerge("flex flex-col gap-2", className)}
    />
  );
}

Form.Field = Field;
Form.Message = Message;
