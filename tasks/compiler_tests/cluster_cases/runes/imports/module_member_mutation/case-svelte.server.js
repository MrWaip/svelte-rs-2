import * as $ from "svelte/internal/server";
import { listener } from "./utils.js";
export function setup() {
	listener.v = () => 1;
}
