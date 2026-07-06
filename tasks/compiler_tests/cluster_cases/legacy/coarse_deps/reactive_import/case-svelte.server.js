import * as $ from "svelte/internal/server";
import { x } from "./x.js";
export default function App($$renderer, $$props) {
	let a = $$props["a"];
	$: {
		a;
		x;
	}
	$.bind_props($$props, { a });
}
