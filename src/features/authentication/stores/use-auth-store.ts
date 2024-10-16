import { User } from "@/types/user";
import { create } from "zustand";

interface AuthState {
  user: User | null;
  isAuthenticated: () => boolean;
  login: (user: User) => void;
  logout: () => void;
}

export const useAuthStore = create<AuthState>()((set, get) => ({
  user: null,
  isAuthenticated: () => !!get().user,
  login: (user) => set({ user }),
  logout: () => set({ user: null }),
}));
