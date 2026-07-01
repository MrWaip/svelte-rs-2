import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	const a = ($$anchor) => {
		$.next();
		var text = $.text();
		text.nodeValue = "a";
		$.append($$anchor, text);
	};
	const b = ($$anchor) => {
		a($$anchor);
	};
	const c = ($$anchor) => {
		b($$anchor);
	};
	let abc = "a";
	c($$anchor);
}
