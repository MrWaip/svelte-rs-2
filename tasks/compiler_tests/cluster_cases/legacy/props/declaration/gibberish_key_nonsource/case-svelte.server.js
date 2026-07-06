import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { "ysc%%gibberish": one } = $$props;
	$$renderer.push(`<!---->${$.escape(one)}`);
}
