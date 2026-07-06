import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { zero } = $$props;
	$$renderer.push(`<!---->${$.escape(zero)}`);
}
