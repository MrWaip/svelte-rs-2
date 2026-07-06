import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let foo = 0;
	let bar = 0;
	foo = 4;
	$: {
		bar = 0;
		outer: for (let i = 0; i < foo; i++) {
			for (let j = 0; j < foo; j++) {
				if (j > i) break outer;
				bar += j;
			}
		}
	}
	$$renderer.push(`<h1>${$.escape(foo)} ${$.escape(bar)}</h1>`);
}
