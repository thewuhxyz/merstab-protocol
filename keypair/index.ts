// import fs from "fs";
import { managerPair } from "./managerpair";
import { depositorPair } from "./depositorpair";
import { depositor2Pair } from "./depositor2pair";

export const manager = Uint8Array.from(
  managerPair
);

export const depositor = Uint8Array.from(
  depositorPair
);

export const depositor2 = Uint8Array.from(
  depositor2Pair
);
