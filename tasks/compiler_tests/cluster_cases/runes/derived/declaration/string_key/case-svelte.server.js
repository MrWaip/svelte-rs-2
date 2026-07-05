import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let src = {
		"a-b": 1,
		"c d": 2
	};
	let ab = $.derived(() => src["a-b"]), cd = $.derived(() => src["c d"]);
	$$renderer.push(`<button>${$.escape(ab())}${$.escape(cd())}</button>`);
}
