App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<img alt="" class="svelte-1qciquw"/>`), App[$.FILENAME], [[3, 8]]);
var root_1 = $.add_locations($.from_html(`<div class="icon-slot svelte-1qciquw"><!></div>`), App[$.FILENAME], [[1, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var div = root_1();
	var node = $.child(div);
	$.slot(node, $$props, "icon", {}, ($$anchor) => {
		var img = root();
		$.append($$anchor, img);
	});
	$.reset(div);
	$.append($$anchor, div);
	return $.pop($$exports);
}
