export function base64Encode(ab: ArrayBufferLike) {
  return btoa(String.fromCharCode(...new Uint8Array(ab)));
}

export async function sha256(str: string) {
  const ab = (new TextEncoder).encode(str);
  return await crypto.subtle.digest('SHA-256', ab);
}
