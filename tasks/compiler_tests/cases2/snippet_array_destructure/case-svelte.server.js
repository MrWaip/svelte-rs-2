import * as $ from "svelte/internal/server";
function show($$renderer, [a, b]) {
	$$renderer.push(`<p>${$.escape(a)} and ${$.escape(b)}</p>`);
}
function withRest($$renderer, [first, ...others]) {
	$$renderer.push(`<p>${$.escape(first)}</p>`);
}
export default function App($$renderer) {
	let pair = [10, 20];
	show($$renderer, pair);
	$$renderer.push(`<!----> `);
	withRest($$renderer, [
		1,
		2,
		3
	]);
	$$renderer.push(`<!---->`);
}
