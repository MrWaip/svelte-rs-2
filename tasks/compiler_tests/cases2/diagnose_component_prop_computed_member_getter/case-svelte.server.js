import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const obj = $.derived(() => ({ "data-testid": "x" }));
	Comp($$renderer, { dataTestid: obj()["data-testid"] });
}
