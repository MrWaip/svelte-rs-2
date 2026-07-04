App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<meta name="description" content="test"/>`), App[$.FILENAME], [[11, 4]]);
var root_1 = $.add_locations($.from_html(`<div> </div>`), App[$.FILENAME], [[16, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = $.tag($.state(0), "count");
	function handleScroll() {
		$.update(count);
	}
	var $$exports = { ...$.legacy_api() };
	var div = root_1();
	$.head("q2w0q4", ($$anchor) => {
		var meta = root();
		$.deferred_template_effect(() => {
			$.document.title = `Count: ${$.get(count) ?? ""}`;
		});
		$.append($$anchor, meta);
	});
	$.event("scroll", $.window, handleScroll);
	var text = $.child(div);
	$.reset(div);
	$.template_effect(() => $.set_text(text, `Count: ${$.get(count) ?? ""}`));
	$.append($$anchor, div);
	return $.pop($$exports);
}
