import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[8, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	async function getPromise() {
		return fetch("/api");
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.async(node, [], [], (node) => {
		$.add_svelte_meta(() => $.await(node, async () => (await $.track_reactivity_loss(getPromise()))(), null, ($$anchor, value) => {
			var p = root();
			var text = $.child(p, true);
			$.reset(p);
			$.template_effect(() => $.set_text(text, $.get(value)));
			$.append($$anchor, p);
		}), "await", App, 7, 0);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
