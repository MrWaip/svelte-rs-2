import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let foo = $$props["foo"];
	$$renderer.push(`<textarea>`);
	const $$body = $.escape(foo);
	if ($$body) {
		$$renderer.push(`${$$body}`);
	} else {}
	$$renderer.push(`</textarea> <textarea>`);
	const $$body_1 = $.escape("hello");
	if ($$body_1) {
		$$renderer.push(`${$$body_1}`);
	} else {}
	$$renderer.push(`</textarea> <textarea>`);
	const $$body_2 = $.escape(`a${$.stringify(foo)}b`);
	if ($$body_2) {
		$$renderer.push(`${$$body_2}`);
	} else {}
	$$renderer.push(`</textarea>`);
	$.bind_props($$props, { foo });
}
