import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p>content</p>`), App[$.FILENAME], [[8, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	async function getValue() {
		return 42;
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.async(node, [], [async () => (await $.track_reactivity_loss(getValue()))()], (node, $$key) => {
		$.add_svelte_meta(() => $.key(node, () => $.get($$key), ($$anchor) => {
			var p = root();
			$.append($$anchor, p);
		}), "key", App, 7, 0);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
