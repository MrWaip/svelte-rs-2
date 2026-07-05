import * as $ from "svelte/internal/server";
function defaultWrapWith($$renderer, mf) {
	mf($$renderer);
	$$renderer.push(`<!---->`);
}
export default function App($$renderer, $$props) {
	let { wrapWith = defaultWrapWith, label = "" } = $$props;
	let count = 0;
	function inner($$renderer) {
		$$renderer.push(`<span>${$.escape(label)}0</span>`);
	}
	$.head("q2w0q4", $$renderer, ($$renderer) => {
		$$renderer.push(`<style>
        :root { --x: red; }
    </style>`);
	});
	$$renderer.push(`<div>`);
	wrapWith($$renderer, inner);
	$$renderer.push(`<!----></div>`);
}
