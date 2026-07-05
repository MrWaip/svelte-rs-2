import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let val = 0;
	;
	;
	$.pop();
}
