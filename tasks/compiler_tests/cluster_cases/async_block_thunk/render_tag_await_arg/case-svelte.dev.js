import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const row = $.wrap_snippet(App, function($$anchor, value = $.noop) {
	$.validate_snippet_args(...arguments);
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, value()));
	$.append($$anchor, p);
});
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[9, 21]]);
var root_1 = $.add_locations($.from_html(`<button>inc</button> <!>`, 1), App[$.FILENAME], [[11, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let x = $.tag($.state(0), "x");
	function delay(value) {
		return Promise.resolve({ value });
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	$.async(node, void 0, [async () => (await $.track_reactivity_loss((await $.save(delay($.get(x))))().value))()], (node, $0) => {
		$.add_svelte_meta(() => row(node, () => $.get($0)), "render", App, 13, 0);
	});
	$.delegated("click", button, function click() {
		return $.update(x);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
