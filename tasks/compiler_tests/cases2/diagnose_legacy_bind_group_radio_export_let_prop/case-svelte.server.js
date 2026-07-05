import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let value = $.fallback($$props["value"], undefined);
	$$renderer.push(`<input type="radio"${$.attr("checked", value === "a", true)} value="a"/>`);
	$.bind_props($$props, { value });
}
