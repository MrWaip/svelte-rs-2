import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let foo = 0;
	let bar;
	foo = 1;
	$: {
		bar = foo + 1;
		if (foo) {
			break $;
		}
		bar = foo + 2;
	}
	$$renderer.push(`<h1>${$.escape(foo)} ${$.escape(bar)}</h1>`);
}
