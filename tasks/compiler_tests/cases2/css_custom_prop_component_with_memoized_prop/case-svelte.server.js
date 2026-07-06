import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
import { thing } from "./lib";
export default function App($$renderer, $$props) {
	let m;
	let p = $$props["p"];
	$: m = p;
	$.css_props($$renderer, true, { "--color": "red" }, () => {
		Child($$renderer, { config: {
			a: m,
			b: thing
		} });
	});
	$.bind_props($$props, { p });
}
