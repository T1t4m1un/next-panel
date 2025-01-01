import { create } from 'zustand';

interface PanelStore {
  isAuth: undefined | boolean,
  setIsAuth: (isAuth: boolean) => void,
  isLanAvailable: boolean,
  setIsLanAvailable: (isAuth: boolean) => void,
}

export const usePanelStore = create<PanelStore>(set => ({
  isAuth: void 0,
  setIsAuth: (isAuth: boolean) => set(() => ({ isAuth })),
  isLanAvailable: false,
  setIsLanAvailable: (isLanAvailable: boolean) => set(() => ({ isLanAvailable })),
}));
