import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
import { obj } from "./x.js";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const y = obj.prop;
	Comp($$anchor, { get p() {
		return y;
	} });
	$.pop();
}
