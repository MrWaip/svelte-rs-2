import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let tmp = {
		"a-b": 1,
		"c d": 2
	}, ab = $.fallback($$props["ab"], () => tmp["a-b"], true), cd = $.fallback($$props["cd"], () => tmp["c d"], true);
	$$renderer.push(`<button>${$.escape(ab)}${$.escape(cd)}</button>`);
	$.bind_props($$props, {
		ab,
		cd
	});
}
