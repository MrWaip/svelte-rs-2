import * as $ from "svelte/internal/server";
import { numbers } from "./data.js";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<p>${$.escape(numbers.join(" + "))}</p>`);
	});
}
