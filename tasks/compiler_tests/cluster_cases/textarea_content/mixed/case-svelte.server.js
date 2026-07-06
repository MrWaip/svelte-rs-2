import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let foo = $$props["foo"];
	$$renderer.push(`<textarea>`);
	const $$body = $.escape(`	<p>not actually an element. ${$.stringify(foo)}</p>
`);
	if ($$body) {
		$$renderer.push(`${$$body}`);
	} else {}
	$$renderer.push(`</textarea>`);
	$.bind_props($$props, { foo });
}
