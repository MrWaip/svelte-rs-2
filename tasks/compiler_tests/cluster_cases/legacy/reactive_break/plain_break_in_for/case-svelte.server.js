import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let foo = 0;
	let bar = 0;
	foo = 5;
	$: {
		bar = 0;
		for (let i = 0; i < foo; i++) {
			if (i > 2) break;
			bar += i;
		}
	}
	$$renderer.push(`<h1>${$.escape(foo)} ${$.escape(bar)}</h1>`);
}
