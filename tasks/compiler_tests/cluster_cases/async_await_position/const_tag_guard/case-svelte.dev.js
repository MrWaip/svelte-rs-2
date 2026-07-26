import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[13, 1]]);
var root_1 = $.add_locations($.from_html(`<button>inc</button> <!>`, 1), App[$.FILENAME], [[9, 0]]);
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
	$.add_svelte_meta(() => $.each(node, 16, () => [1], $.index, ($$anchor, item) => {
		let current;
		var promises = $.run([async () => current = $.tag((await $.save($.async_derived(async () => (await $.save(delay($.get(x))))().value)))(), "current")]);
		var p = root();
		var text = $.child(p);
		$.reset(p);
		$.template_effect(() => $.set_text(text, `${$.get(current) ?? ""}${item ?? ""}`), void 0, void 0, [promises[0]]);
		$.append($$anchor, p);
	}), "each", App, 11, 0);
	$.delegated("click", button, function click() {
		return $.update(x);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
