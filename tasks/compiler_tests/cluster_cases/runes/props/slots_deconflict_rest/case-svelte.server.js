import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const $$slots = $.sanitize_slots($$props);
	let { a, $$slots: $$slots_, $$events, ...rest } = $$props;
	$$renderer.push(`<p>${$.escape(a)} ${$.escape(Object.keys(rest))}</p> `);
	if ($$slots.foo) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`<p>foo exists</p>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
