import * as $ from "svelte/internal/server";
import { noop } from "./helpers.js";
function foo($$renderer) {
	$$renderer.push(`<!---->${$.escape(noop)}`);
}
export default function App($$renderer) {
	foo($$renderer);
}
