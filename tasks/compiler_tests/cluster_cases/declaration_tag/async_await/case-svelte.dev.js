import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[3, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 16, () => items, $.index, ($$anchor, item) => {
		let value;
		var promises = $.run([async () => value = (await $.track_reactivity_loss(item.load()))()]);
		var p = root();
		var text = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text, value), void 0, void 0, [promises[0]]);
		$.append($$anchor, p);
	}), "each", App, 1, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
