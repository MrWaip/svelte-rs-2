import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let a, b;
	function makePair() {
		return [1, 2];
	}
	const pair = makePair();
	$: [a, b] = pair;
	$$renderer.push(`<!---->${$.escape(a)}${$.escape(b)}`);
}
