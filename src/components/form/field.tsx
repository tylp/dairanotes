import * as RadixForm from "@radix-ui/react-form";
import { FieldLabel } from "./field-label";
import { FieldControl } from "./field-control";
import { Message } from "./message";
import { twMerge } from "tailwind-merge";

export function Field({ className, ...props }: RadixForm.FormFieldProps) {
  return (
    <RadixForm.Field
      {...props}
      className={twMerge("flex flex-col gap-2", className)}
    />
  );
}

Field.Label = FieldLabel;
Field.Control = FieldControl;
Field.Message = Message;
