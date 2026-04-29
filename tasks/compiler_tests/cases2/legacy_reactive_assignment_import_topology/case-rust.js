import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import data from "./dep.js";
var $$_import_data = $.reactive_import(() => data);
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const doubled = $.mutable_source();
	const total = $.mutable_source();
	function bump() {
		$$_import_data($$_import_data().count += 1);
	}
	$.legacy_pre_effect(() => $$_import_data(), () => {
		$.set(total, $$_import_data().count);
	});
	$.legacy_pre_effect(() => $.get(total), () => {
		$.set(doubled, $.get(total) * 2);
	});
	$.legacy_pre_effect_reset();
	$.init();
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(doubled)));
	$.delegated("click", button, bump);
	$.append($$anchor, button);
	$.pop();
}
$.delegate(["click"]);
