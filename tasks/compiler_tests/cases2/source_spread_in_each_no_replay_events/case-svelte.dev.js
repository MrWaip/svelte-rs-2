App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<source/>`), App[$.FILENAME], [[7, 8]]);
var root_1 = $.add_locations($.from_html(`<picture></picture>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var picture = root_1();
	$.add_svelte_meta(() => $.each(picture, 21, () => $$props.sources, $.index, ($$anchor, source) => {
		var source_1 = root();
		$.attribute_effect(source_1, () => ({ ...$.get(source) }));
		$.append($$anchor, source_1);
	}), "each", App, 6, 4);
	$.reset(picture);
	$.append($$anchor, picture);
	return $.pop($$exports);
}
