import { getLocalFingerprint, getLocalPublicKey } from './local';

function str2ab(str: string): ArrayBuffer {
  const binaryString = atob(str);
  const length = binaryString.length;
  const arrayBuffer = new ArrayBuffer(length);
  const view = new Uint8Array(arrayBuffer);
  for (let i = 0; i < length; i++) {
    view[i] = binaryString.charCodeAt(i);
  }
  return arrayBuffer;
}

async function importPemPublicKey(pemKey: string) {
  const binaryDer = str2ab(pemKey.replace(/-----BEGIN PUBLIC KEY-----|\n|-----END PUBLIC KEY-----/g, ''));
  return crypto.subtle.importKey(
    'spki', // 公钥格式
    binaryDer,
    {
      name: 'RSASSA-PKCS1-v1_5',  // 使用 PSS 填充模式
      hash: { name: 'SHA-256' },
    },
    true,
    ['verify'],
  );
}

async function verifySignature(origin: string, signature: ArrayBuffer, public_key: CryptoKey) {
  const dataBuffer = (new TextEncoder).encode(origin);
  try {
    return crypto.subtle.verify(
      { name: 'RSASSA-PKCS1-v1_5' },
      public_key,
      signature,
      dataBuffer,
    );
  } catch {
    return false;
  }
}

async function getPublicKey(): Promise<CryptoKey> {
  const public_key = await getLocalPublicKey();
  return await importPemPublicKey(public_key);
}

export async function localVerify(): Promise<boolean> {
  try {
    const b64Token = localStorage.getItem('local_token');
    if (b64Token === null) {
      return false;
    }
    const token = Uint8Array.from(atob(b64Token));

    const public_key = await getPublicKey();

    const fp = await getLocalFingerprint();

    try {
      return verifySignature(fp, token, public_key);
    } catch {
      return false;
    }
  } catch {
    return false;
  }
}
