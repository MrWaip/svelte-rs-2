import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let onclick = $.fallback($$props["onclick"], undefined);
	let useFn = $.fallback($$props["useFn"], undefined);
	let useArgs = $.fallback($$props["useArgs"], () => [], true);
	let href = $.fallback($$props["href"], undefined);
	function getTag() {
		return href ? "a" : "div";
	}
	$.element($$renderer, getTag(), () => {
		$$renderer.push(`${$.attr("href", href)}`);
	}, () => {
		$$renderer.push(`x`);
	});
	$.bind_props($$props, {
		onclick,
		useFn,
		useArgs,
		href
	});
}
