import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<div>fallback html</div>`);
var root = $.from_mathml(`<annotation-xml><!></annotation-xml>`, 2);
export default function App($$anchor) {
	let shown = true;
	var annotation_xml = root();
	var node = $.child(annotation_xml);
	{
		var consequent = ($$anchor) => {
			var div = root_1();
			$.append($$anchor, div);
		};
		$.if(node, ($$render) => {
			if (shown) $$render(consequent);
		});
	}
	$.reset(annotation_xml);
	$.append($$anchor, annotation_xml);
}
