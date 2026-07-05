import * as $ from "svelte/internal/server";
import foo from "./foo.js";
foo.bar = "baz";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<p>${$.escape(foo.bar)}</p>`);
	});
}
