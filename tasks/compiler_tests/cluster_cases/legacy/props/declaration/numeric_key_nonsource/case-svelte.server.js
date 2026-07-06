import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { 0: zero } = $$props;
	$$renderer.push(`<!---->${$.escape(zero)}`);
}
