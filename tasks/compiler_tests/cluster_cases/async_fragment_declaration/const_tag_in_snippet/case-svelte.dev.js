import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[7, 1]]);
var root_1 = $.add_locations($.from_html(`<!> <button>go</button>`, 1), App[$.FILENAME], [[11, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const row = $.wrap_snippet(App, function($$anchor) {
		$.validate_snippet_args(...arguments);
		let a;
		var promises = $.run([async () => a = $.tag((await $.save($.async_derived(async () => (await $.save(Promise.resolve($.get(n))))())))(), "a")]);
		var p = root();
		var text = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text, $.get(a)), void 0, void 0, [promises[0]]);
		$.append($$anchor, p);
	});
	let n = $.tag($.state(1), "n");
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => row(node), "render", App, 10, 0);
	var button = $.sibling(node, 2);
	$.delegated("click", button, function click() {
		return $.update(n);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
