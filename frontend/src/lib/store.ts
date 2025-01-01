import { create } from 'zustand';

interface PanelStore {
  isAuth: undefined | boolean,
  setIsAuth: (isAuth: boolean) => void,
}

export const usePanelStore = create<PanelStore>(set => ({
  isAuth: void 0,
  setIsAuth: (isAuth: boolean) => set(() => ({ isAuth })),
}));
