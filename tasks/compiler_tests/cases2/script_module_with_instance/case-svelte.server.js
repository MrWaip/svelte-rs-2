import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export const theme = writable("light");
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let count = 0;
		$$renderer.push(`<button>${$.escape(count)}</button>`);
	});
}
