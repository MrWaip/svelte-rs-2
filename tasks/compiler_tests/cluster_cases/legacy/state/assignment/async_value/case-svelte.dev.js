import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let a = $.tag($.mutable_source(0), "a");
	let b = $.tag($.mutable_source(0), "b");
	async function update() {
		await (async ($$value) => {
			var $$array = $.to_array($$value, 2);
			$.set(a, $$array[0]);
			$.set(b, $$array[1]);
		})([1, (await $.track_reactivity_loss(Promise.resolve(2)))()]);
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(b) ?? ""}`));
	$.event("click", button, update);
	$.append($$anchor, button);
	return $.pop($$exports);
}
