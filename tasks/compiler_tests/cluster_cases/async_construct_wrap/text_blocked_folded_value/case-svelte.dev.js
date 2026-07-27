import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>inc</button> <p> </p>`, 1), App[$.FILENAME], [[9, 0], [10, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = $.tag($.state(1), "count");
	async function getDouble(value) {
		return value * 2;
	}
	var double;
	var $$promises = $.run([async () => double = await $.async_derived(async () => (await $.track_reactivity_loss(getDouble($.get(count))))(), "double", "(unknown):6:16")]);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var button = $.first_child(fragment);
	var p = $.sibling(button, 2);
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `Count: ${$.get(count) ?? ""} Double: ${$.get(double) ?? ""}`), void 0, void 0, [$$promises[0]]);
	$.delegated("click", button, function click() {
		return $.update(count);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
