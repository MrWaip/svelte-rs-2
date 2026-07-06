import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let href = $.fallback($$props["href"], undefined);
	function getTag() {
		return href ? "a" : "div";
	}
	function getRole() {
		return href ? "link" : undefined;
	}
	$.element($$renderer, getTag(), () => {
		$$renderer.push(`${$.attr("role", getRole())}${$.attr("href", href)}`);
	}, () => {
		$$renderer.push(`x`);
	});
	$.bind_props($$props, { href });
}
