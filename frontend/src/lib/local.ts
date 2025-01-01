import fpPromise from '@fingerprintjs/fingerprintjs';
import { api } from './api';

export async function getLocalAddr(): Promise<string> {
  let local_addr = localStorage.getItem('local_addr');
  if (local_addr == null) {
    local_addr = await api.local.getAddr();
    localStorage.setItem('local_addr', local_addr);
  }
  return local_addr;
}

export async function getLocalPublicKey(): Promise<string> {
  let public_key = localStorage.getItem('public_key');
  if (public_key === null) {
    try {
      public_key = await api.local.getPublicKey();
      localStorage.setItem('public_key', public_key);
    } catch {
      throw Error('Cannot get public key to authenticate');
    }
  }
  return public_key;
}


declare global {
  // eslint-disable-next-line no-var
  var fp: string;
}

export async function getLocalFingerprint(): Promise<string> {
  if (globalThis.fp) {
    return globalThis.fp as string;
  }

  return await fpPromise.load()
    .then(fp => fp.get())
    .then(res => {
      const fp = res.visitorId;
      globalThis.fp = fp;
      return fp;
    });
}