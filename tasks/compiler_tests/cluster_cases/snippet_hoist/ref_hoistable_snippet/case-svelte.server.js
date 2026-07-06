import * as $ from "svelte/internal/server";
import { noop } from "./helpers.js";
function a($$renderer) {
	$$renderer.push(`<!---->${$.escape(noop)}`);
}
function b($$renderer) {
	a($$renderer);
}
export default function App($$renderer) {
	b($$renderer);
}
