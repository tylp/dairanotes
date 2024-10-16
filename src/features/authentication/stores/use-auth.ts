import { User } from "@/types/user";
import { create } from "zustand";

interface AuthState {
  user: User | null;
  isAuthenticated: boolean;
  login: (user: User) => void;
  logout: () => void;
}

export const useAuthStore = create<AuthState>()((set) => ({
  user: null,
  get isAuthenticated() {
    return !!this.user;
  },
  login: (user) => set({ user }),
  logout: () => set({ user: null }),
}));
