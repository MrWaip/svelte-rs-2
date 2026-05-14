import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	const obj = $.derived(() => ({ "data-testid": "x" }));
	Comp($$anchor, { get dataTestid() {
		return $.get(obj)["data-testid"];
	} });
}
