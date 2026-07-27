import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>inc</button> <p> </p>`, 1), App[$.FILENAME], [[7, 0], [8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = $.tag($.state(0), "count");
	var delayed, doubled;
	var $$promises = $.run([async () => delayed = await $.async_derived(async () => (await $.track_reactivity_loss(Promise.resolve($.get(count))))(), "delayed", "(unknown):3:17"), () => doubled = $.tag($.derived(() => $.get(count) * 2), "doubled")]);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var button = $.first_child(fragment);
	var p = $.sibling(button, 2);
	var text = $.child(p);
	$.reset(p);
	$.template_effect(($0) => $.set_text(text, `${$.get(delayed) ?? ""}${$0 ?? ""}`), [() => $.strict_equals($.eager(() => $.get(doubled)), $.get(doubled), false)], void 0, [$$promises[0], $$promises[1]]);
	$.delegated("click", button, function click() {
		return $.update(count);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
