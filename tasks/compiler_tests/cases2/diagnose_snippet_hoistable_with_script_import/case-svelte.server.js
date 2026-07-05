import * as $ from "svelte/internal/server";
import { noop } from "./helpers.js";
function socket($$renderer) {
	$$renderer.push(`<div>${$.escape(noop)}</div>`);
}
export default function App($$renderer) {
	socket($$renderer);
}
