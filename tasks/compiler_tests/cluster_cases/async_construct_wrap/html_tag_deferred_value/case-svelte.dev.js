import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div><!></div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var html;
	var $$promises = $.run([async () => html = (await $.track_reactivity_loss(Promise.resolve("<b>hi</b>")))()]);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	var node = $.child(div);
	$.async(node, [$$promises[0]], void 0, (node) => {
		$.html(node, () => html);
	});
	$.reset(div);
	$.append($$anchor, div);
	return $.pop($$exports);
}
