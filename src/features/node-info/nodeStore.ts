import { atom } from "nanostores";

export const $nodePeerId = atom<string | null>(null);
export const $nodeVersion = atom<string | null>(null);
export const $nodeAddresses = atom<string[]>([]);