import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[10, 1]]);
var root_1 = $.add_locations($.from_html(`<!> <button>inc</button>`, 1), App[$.FILENAME], [[14, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const row = $.wrap_snippet(App, function($$anchor) {
		$.validate_snippet_args(...arguments);
		var p = root();
		var text = $.child(p, true);
		$.reset(p);
		$.template_effect(($0) => $.set_text(text, $0), void 0, [async () => (await $.track_reactivity_loss(compute(count)))()]);
		$.append($$anchor, p);
	});
	let count = 0;
	async function compute(v) {
		return v * 2;
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => row(node), "render", App, 13, 0);
	var button = $.sibling(node, 2);
	$.delegated("click", button, function click() {
		return count++;
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
