import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<li> </li>`), App[$.FILENAME], [[7, 2]]);
var root_1 = $.add_locations($.from_html(`<ul><!></ul>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let deferred = $.tag_proxy($.proxy(Promise.withResolvers()), "deferred");
	var $$exports = { ...$.legacy_api() };
	var ul = root_1();
	var node = $.child(ul);
	$.async(node, [], [async () => (await $.track_reactivity_loss(deferred.promise))()], (node, $$collection) => {
		$.add_svelte_meta(() => $.each(node, 17, () => $.get($$collection), $.index, ($$anchor, item) => {
			var li = root();
			var text = $.child(li, true);
			$.reset(li);
			$.template_effect(() => $.set_text(text, $.get(item)));
			$.append($$anchor, li);
		}), "each", App, 6, 1);
	});
	$.reset(ul);
	$.append($$anchor, ul);
	return $.pop($$exports);
}
