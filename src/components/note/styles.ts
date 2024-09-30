import { cva } from "class-variance-authority";

export const noteStyles = cva("", {
  variants: {
    color: {
      red: "bg-[#fca590]",
      orange: "bg-[#f8e7b4]",
      blue: "bg-[#cdeff1]",
      green: "bg-[#cdf1d0]",
      yellow: "bg-[#ffffa0]",
    },
    padding: {
      none: "p-0 rounded-md",
      sm: "p-2 rounded-lg",
      md: "p-4 rounded-xl",
      lg: "p-6 rounded-2xl",
    },
  },
});
