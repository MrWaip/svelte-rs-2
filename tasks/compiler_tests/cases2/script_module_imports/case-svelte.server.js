import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
const theme = writable("light");
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let count = 0;
		$$renderer.push(`<p>0</p>`);
	});
}
