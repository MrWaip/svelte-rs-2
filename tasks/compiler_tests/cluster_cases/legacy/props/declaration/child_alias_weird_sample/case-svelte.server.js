import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { 0: zero, "ysc%%gibberish": one } = $$props;
	$$renderer.push(`<!---->${$.escape(zero)} ${$.escape(one)}`);
}
