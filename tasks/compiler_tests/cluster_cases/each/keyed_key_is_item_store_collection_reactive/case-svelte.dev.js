App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div> </div>`), App[$.FILENAME], [[7, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const $s = () => ($.validate_store($.get(s), "s"), $.store_get($.get(s), "$s", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const s = $.tag($.derived(() => $$props.store), "s");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, () => $s().keys, (key) => key, ($$anchor, key) => {
		const column = $.tag($.derived(() => $$props.columns[$.get(key)]), "column");
		$.get(column);
		var div = root();
		var text = $.child(div, true);
		$.reset(div);
		$.template_effect(() => $.set_text(text, $.get(column)));
		$.append($$anchor, div);
	}), "each", App, 5, 0);
	$.append($$anchor, fragment);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
