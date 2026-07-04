import * as $ from "svelte/internal/client";
import { listener } from "./utils.js";
export function setup() {
	listener.v = () => 1;
}
