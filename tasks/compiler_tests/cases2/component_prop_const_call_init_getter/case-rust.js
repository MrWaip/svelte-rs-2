import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
import { foo } from "./x.js";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const editUrl = foo();
	Comp($$anchor, {
		get href() {
			return editUrl;
		},
		radius: 16
	});
	$.pop();
}
