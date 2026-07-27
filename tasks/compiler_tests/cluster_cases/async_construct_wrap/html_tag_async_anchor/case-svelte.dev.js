import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<h1><!></h1>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let deferred = $.tag_proxy($.proxy(Promise.withResolvers()), "deferred");
	var $$exports = { ...$.legacy_api() };
	var h1 = root();
	var node = $.child(h1);
	$.async(node, [], [async () => (await $.track_reactivity_loss(deferred.promise))()], (node, $$html) => {
		$.html(node, () => $.get($$html));
	});
	$.reset(h1);
	$.append($$anchor, h1);
	return $.pop($$exports);
}
