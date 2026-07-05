import * as $ from "svelte/internal/client";
const greeting = ($$anchor) => {
	var p = root();
	$.append($$anchor, p);
};
var root = $.from_html(`<p>Hello</p>`);
export default function App($$anchor) {
	greeting?.($$anchor);
}
