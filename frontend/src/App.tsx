import './App.css';
import { Panel } from './view/panel';
import { Login } from './view/login';
import { Pending } from './view/pending';

import { useEffect } from 'react';
import { localVerify } from './lib/auth';
import { usePanelStore } from './lib/store';
import { getLocalAddr } from './lib/local';

function App() {
  const isAuth = usePanelStore(state => state.isAuth);
  const setIsAuth = usePanelStore().setIsAuth;
  const setIsLanAvailabe = usePanelStore().setIsLanAvailable;

  useEffect(() => {
    (async () => setIsAuth(await localVerify())) ();
  }, [setIsAuth]);

  useEffect(() => {
    const interval = setInterval(() => {
      (async () => {
        const local_addr = await getLocalAddr();
        await fetch(`http://${local_addr}/api/ping`, { method: 'POST' })
          .then(val => val.text())
          .then(text => setIsLanAvailabe(text === 'pong'))
          .catch(() => setIsLanAvailabe(false));
      }) ();
    }, 10 * 1000);

    return () => clearInterval(interval);
  }, [setIsLanAvailabe]);

  if (isAuth === void 0) {
    return <Pending />;
  } else if (isAuth) {
    return <Panel />;
  } else {
    return <Login />;
  }
}

export default App;
