import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let test = $.fallback($$props["test"], () => [], true);
	$$renderer.push(`<label>a <input type="checkbox"${$.attr("checked", test.includes("a"), true)} value="a"/></label>`);
	$.bind_props($$props, { test });
}
