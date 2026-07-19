import * as $ from "svelte/internal/server";
import { noop } from "./x.js";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let count = 0;
		noop(count);
		$$renderer.push(`<button>${$.escape(count)}</button>`);
	});
}
