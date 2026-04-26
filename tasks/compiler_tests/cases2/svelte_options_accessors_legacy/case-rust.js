import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p></p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let count = 1;
	var $$exports = {
		get count() {
			return count();
		},
		set count($$value) {
			count($$value);
			$.flush();
		}
	};
	var p = root();
	p.textContent = count;
	$.append($$anchor, p);
	return $.pop($$exports);
}
