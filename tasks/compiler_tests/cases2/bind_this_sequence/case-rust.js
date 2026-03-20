import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	let ref = $.state(void 0);
	function set(el) {
		$.set(ref, el, true);
	}
	function get() {
		return $.get(ref);
	}
	var div = root();
	$.bind_this(div, (_) => get(), () => set(el));
	$.append($$anchor, div);
}
