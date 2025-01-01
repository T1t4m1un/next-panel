import './App.css';
import { Panel } from './view/panel';
import { Login } from './view/login';
import { Pending } from './view/pending';

import { useEffect } from 'react';
import { localVerify } from './lib/auth';
import { usePanelStore } from './lib/store';

function App() {
  const isAuth = usePanelStore(state => state.isAuth);
  const setIsAuth = usePanelStore().setIsAuth;

  useEffect(() => {
    (async () => setIsAuth(await localVerify())) ();
  }, [setIsAuth]);

  if (isAuth === void 0) {
    return <Pending />;
  } else if (isAuth) {
    return <Panel />;
  } else {
    return <Login />;
  }
}

export default App;
