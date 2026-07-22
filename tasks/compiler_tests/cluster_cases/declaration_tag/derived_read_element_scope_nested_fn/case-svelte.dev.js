App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div> </div>`), App[$.FILENAME], [[1, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	{
		let dt = $.tag($.derived(() => [
			1,
			2,
			3
		]), "dt");
		var text = $.child(div);
		$.reset(div);
		$.template_effect(($0) => $.set_text(text, `${$.get(dt).length ?? ""}
	${$0 ?? ""}`), [() => $.get(dt).map((x) => x + $.get(dt).length)]);
	}
	$.append($$anchor, div);
	return $.pop($$exports);
}
