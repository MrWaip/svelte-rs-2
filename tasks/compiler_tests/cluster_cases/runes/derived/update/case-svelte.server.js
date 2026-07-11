import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = $.derived(() => 0);
	let postfix = $.update_derived(count);
	let postfix_minus = $.update_derived(count, -1);
	let prefix = $.update_derived_pre(count);
	let prefix_minus = $.update_derived_pre(count, -1);
	$$renderer.push(`<p>${$.escape(postfix)}, ${$.escape(postfix_minus)}, ${$.escape(prefix)}, ${$.escape(prefix_minus)}, ${$.escape(count())}</p>`);
}
