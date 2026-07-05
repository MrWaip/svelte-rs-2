import * as $ from "svelte/internal/server";
import { api } from "./api.js";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		function doSomething() {
			api.call();
		}
		$$renderer.push(`<button>click</button>`);
	});
}
