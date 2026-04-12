import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p></p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let label = $.prop($$props, "label", 8, "sum");
	let a = 1;
	let b = 2;
	$: console.log(`${label()}: ${sum}`);
	$: sum = a + b;
	$: ((param) => {
		via_iife = param * 2;
	})(sum);
	var p = root();
	p.textContent = `${sum ?? ""}-${via_iife ?? ""}`;
	$.append($$anchor, p);
	$.pop();
}
