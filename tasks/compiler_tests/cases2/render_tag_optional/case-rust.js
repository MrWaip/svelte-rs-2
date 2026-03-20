import * as $ from "svelte/internal/client";
const greeting = ($$anchor) => {
	var p = root_1();
	$.append($$anchor, p);
};
var root_1 = $.from_html(`<p>Hello</p>`);
export default function App($$anchor) {
	greeting?.($$anchor);
}
