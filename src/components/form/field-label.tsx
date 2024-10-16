import * as RadixForm from "@radix-ui/react-form";
import { twMerge } from "tailwind-merge";

export function FieldLabel({ className, ...props }: RadixForm.FormLabelProps) {
  return (
    <RadixForm.Label {...props} className={twMerge("font-bold", className)} />
  );
}
