import { cloneElement, forwardRef } from "react";
import * as RadixForm from "@radix-ui/react-form";
import { TextInput } from "../text-input";

export const FieldControl = forwardRef<
  HTMLInputElement,
  RadixForm.FormControlProps
>(({ children, ...props }, ref) => {
  return (
    <RadixForm.Control asChild {...props}>
      {children ? (
        cloneElement(children as React.ReactElement, { ref })
      ) : (
        <TextInput ref={ref} />
      )}
    </RadixForm.Control>
  );
});

FieldControl.displayName = "FieldControl";
