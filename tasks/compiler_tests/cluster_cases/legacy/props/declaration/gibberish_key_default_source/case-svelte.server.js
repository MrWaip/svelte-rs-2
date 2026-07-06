import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { "a-b": x = 1 } = $$props;
	$$renderer.push(`<!---->${$.escape(x)}`);
}
