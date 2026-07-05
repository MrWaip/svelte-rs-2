App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let ref = $.tag($.state(void 0), "ref");
	function set(el) {
		$.set(ref, el, true);
	}
	function get() {
		return $.get(ref);
	}
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.bind_this(div, (_) => get(), () => set(el));
	$.append($$anchor, div);
	return $.pop($$exports);
}
