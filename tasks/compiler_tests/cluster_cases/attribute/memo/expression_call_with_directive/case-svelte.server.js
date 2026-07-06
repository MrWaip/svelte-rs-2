import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let status = $.fallback($$props["status"], "neutral");
	let disabled = $.fallback($$props["disabled"], false);
	function classify(s) {
		return s + "-x";
	}
	$$renderer.push(`<div${$.attr_class($.clsx(classify(status)), void 0, { "disabled": disabled })}></div>`);
	$.bind_props($$props, {
		status,
		disabled
	});
}
