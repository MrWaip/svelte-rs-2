import * as $ from "svelte/internal/server";
import { transform } from "./utils.js";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let value = 0;
		const fn = $.derived(() => transform(value));
		$$renderer.push(`<div class="output">${$.escape(fn()(value))}</div>`);
	});
}
