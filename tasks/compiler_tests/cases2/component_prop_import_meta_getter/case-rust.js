import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	Comp($$anchor, { get url() {
		return import.meta.env.VITE_X;
	} });
	$.pop();
}
