import * as $ from "svelte/internal/server";
function defaultWrapWith($$renderer, mf) {
	mf($$renderer);
	$$renderer.push(`<!---->`);
}
export default function App($$renderer, $$props) {
	let { wrapWith = defaultWrapWith } = $$props;
	$$renderer.push(`<div>x</div>`);
}
