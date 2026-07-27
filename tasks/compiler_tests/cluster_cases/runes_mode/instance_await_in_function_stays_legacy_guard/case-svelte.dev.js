import "svelte/internal/flags/legacy";
import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p> <button>load</button>`, 1), App[$.FILENAME], [[9, 0], [10, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let count = $.tag($.mutable_source(0), "count");
	async function load() {
		$.set(count, (await $.track_reactivity_loss(Promise.resolve(1)))());
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var p = $.first_child(fragment);
	var text = $.child(p, true);
	$.reset(p);
	var button = $.sibling(p, 2);
	$.template_effect(() => $.set_text(text, $.get(count)));
	$.delegated("click", button, load);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
