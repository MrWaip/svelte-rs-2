import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { o } = $$props;
	let tmp = o, a = tmp.a, b = tmp.b;
	$$renderer.push(`<p>${$.escape(a)} ${$.escape(b)}</p>`);
}
