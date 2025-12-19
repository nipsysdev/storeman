import { atom } from "nanostores";
import { NodeInfo } from "./nodeTypes";

export const $nodeInfo = atom<NodeInfo | null>(null);