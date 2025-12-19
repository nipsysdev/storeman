import { atom } from "nanostores";
import type { NodeInfo } from "./nodeTypes";

export const $nodeInfo = atom<NodeInfo | null>(null);