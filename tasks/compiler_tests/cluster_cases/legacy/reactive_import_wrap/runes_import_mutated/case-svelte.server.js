import * as $ from "svelte/internal/server";
import foo from "./foo.js";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		function add() {
			foo.bar = "baz";
		}
		$$renderer.push(`<p>${$.escape(foo.bar)}</p> <button>add</button>`);
	});
}
