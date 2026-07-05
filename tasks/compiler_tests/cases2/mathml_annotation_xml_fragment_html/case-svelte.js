import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>fallback html</div>`);
var root_1 = $.from_mathml(`<annotation-xml><!></annotation-xml>`, 2);
export default function App($$anchor) {
	let shown = true;
	var annotation_xml = root_1();
	var node = $.child(annotation_xml);
	{
		var consequent = ($$anchor) => {
			var div = root();
			$.append($$anchor, div);
		};
		$.if(node, ($$render) => {
			if (shown) $$render(consequent);
		});
	}
	$.reset(annotation_xml);
	$.append($$anchor, annotation_xml);
}
