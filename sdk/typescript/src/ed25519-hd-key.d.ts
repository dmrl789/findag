declare module 'ed25519-hd-key' {
  export function derivePath(path: string, seed: string): { key: Buffer; chainCode: Buffer };
} 