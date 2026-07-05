import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let action = $$props["action"];
	let onClick = $$props["onClick"];
	let el;
	$$renderer.push(`<div><!--[-->`);
	$.slot($$renderer, $$props, "default", {}, null);
	$$renderer.push(`<!--]--></div>`);
	$.bind_props($$props, {
		action,
		onClick
	});
}
