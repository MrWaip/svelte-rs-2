import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let first, second, total, b, d, e;
	let source = $$props["source"];
	$: ({users: [{name: first}, {name: second}], total} = source);
	$: ({a: [b, {c: [d, e]}]} = source);
	$$renderer.push(`<p>${$.escape(first)}-${$.escape(second)}-${$.escape(total)}-${$.escape(b)}-${$.escape(d)}-${$.escape(e)}</p>`);
	$.bind_props($$props, { source });
}
