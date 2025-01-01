import { api } from '../../lib/api';
import { base64Encode, sha256 } from '../../lib/crypto';
import { getLocalFingerprint } from '../../lib/local';
import { usePanelStore } from '../../lib/store';
import './style.css';
import { FC, useState } from 'react';

export const Login: FC = () => {
  const [username, setUsername] = useState<string>();
  const [password, setPassword] = useState<string>();
  const [isPending, setIsPending] = useState<boolean>(false);
  const setIsAuth = usePanelStore().setIsAuth;

  const handleLogin = () => {
    if (username === undefined || password === undefined) {
      return;
    }
    setIsPending(true);
    (async () => {
      const s256_pwd = base64Encode(await sha256(password));
      const fp = await getLocalFingerprint();
      const res = await api.account.login(username, s256_pwd, fp);
      if (res.status === 'success') {
        localStorage.setItem('token', res.token);
        localStorage.setItem('info', JSON.stringify(res.info));
        setIsAuth(true);
      }
      setIsPending(false);
    }) ();
  };

  return (
    <div className='login-view'>
      <div className='login-panel'>
        <h2>登陆 Next Panel</h2>

        <table>
          <tr>
            <td><label>用户名</label></td>
            <td><input type='text' value={username} onChange={e => setUsername(e.target.value)} /></td>
          </tr>
          <tr>
            <td><label>密码</label></td>
            <td><input type='password' value={password} onChange={e => setPassword(e.target.value)} /></td>
          </tr>
        </table>
        <button onClick={handleLogin} disabled={isPending}>登陆</button>
      </div>
    </div>
  );
};