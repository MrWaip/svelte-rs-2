import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	const foo = ($$anchor) => {
		$.next();
		var text = $.text();
		text.nodeValue = "x";
		$.append($$anchor, text);
	};
	let name = "x";
	foo($$anchor);
}
