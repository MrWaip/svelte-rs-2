import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	let val = "25";
	$$renderer.push(`<div>`);
	$.css_props($$renderer, true, { "--color": `px ${$.stringify(val)}` }, () => {
		Child($$renderer, {});
	});
	$$renderer.push(`</div> <button>x</button>`);
}
