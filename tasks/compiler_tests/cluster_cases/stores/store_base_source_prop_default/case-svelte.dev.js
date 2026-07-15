App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div> </div>`), App[$.FILENAME], [[4, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const $selectionWStore = () => ($.validate_store(selectionWStore(), "selectionWStore"), $.store_get(selectionWStore(), "$selectionWStore", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	let selectionWStore = $.prop($$props, "selectionWStore", 3, undefined);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	var text = $.child(div, true);
	$.reset(div);
	$.template_effect(() => $.set_text(text, $selectionWStore()?.value));
	$.append($$anchor, div);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
