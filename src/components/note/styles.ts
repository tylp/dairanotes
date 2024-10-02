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
      none: "rounded-md p-0",
      sm: "rounded-lg p-2",
      md: "rounded-xl p-4",
      lg: "rounded-2xl p-6",
    },
  },
});
