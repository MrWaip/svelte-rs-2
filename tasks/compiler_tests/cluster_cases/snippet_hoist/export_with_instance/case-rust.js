import * as $ from "svelte/internal/client";
const foo = ($$anchor) => {
	$.next();
	var text = $.text("oo");
	$.append($$anchor, text);
};
export { foo };
var root = $.from_html(`<h1></h1>`);
export default function App($$anchor) {
	let name = "world";
	var h1 = root();
	h1.textContent = "Hello world!";
	$.append($$anchor, h1);
}
