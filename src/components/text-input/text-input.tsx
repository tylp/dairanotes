import React, { forwardRef } from "react";

type TextInputProps = React.InputHTMLAttributes<HTMLInputElement>;

export const TextInput = forwardRef<HTMLInputElement, TextInputProps>(
  (props, ref) => {
    return <input ref={ref} className="border" type="text" {...props} />;
  },
);

TextInput.displayName = "TextInput";
