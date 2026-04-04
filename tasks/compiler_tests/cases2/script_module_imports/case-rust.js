import { writable } from "svelte/store";
import * as $ from "svelte/internal/client";
const theme = writable("light");
var root = $.from_html(`<p></p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let count = 0;
	var p = root();
	p.textContent = "0";
	$.append($$anchor, p);
	$.pop();
}
