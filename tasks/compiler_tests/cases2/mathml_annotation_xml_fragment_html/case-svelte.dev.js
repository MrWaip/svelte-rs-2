App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div>fallback html</div>`), App[$.FILENAME], [[9, 2]]);
var root_1 = $.add_locations($.from_mathml(`<annotation-xml><!></annotation-xml>`, 2), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let shown = true;
	var $$exports = { ...$.legacy_api() };
	var annotation_xml = root_1();
	var node = $.child(annotation_xml);
	{
		var consequent = ($$anchor) => {
			var div = root();
			$.append($$anchor, div);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (shown) $$render(consequent);
		}), "if", App, 8, 1);
	}
	$.reset(annotation_xml);
	$.append($$anchor, annotation_xml);
	return $.pop($$exports);
}
