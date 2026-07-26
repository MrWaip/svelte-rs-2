import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>inc</button> <!>`, 1), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const row = $.wrap_snippet(App, function($$anchor) {
		$.validate_snippet_args(...arguments);
		$.next();
		var text = $.text();
		$.template_effect(($0) => $.set_text(text, $0), void 0, [async () => (await $.track_reactivity_loss(delay($.get(x))))()]);
		$.append($$anchor, text);
	});
	let x = $.tag($.state(0), "x");
	function delay(value) {
		return Promise.resolve(value);
	}
	var $$exports = { ...$.legacy_api() };
	var fragment_1 = root();
	var button = $.first_child(fragment_1);
	var node = $.sibling(button, 2);
	$.add_svelte_meta(() => row(node), "render", App, 13, 0);
	$.delegated("click", button, function click() {
		return $.update(x);
	});
	$.append($$anchor, fragment_1);
	return $.pop($$exports);
}
$.delegate(["click"]);
