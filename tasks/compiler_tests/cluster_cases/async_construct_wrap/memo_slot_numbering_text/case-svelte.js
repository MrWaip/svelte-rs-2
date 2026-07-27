import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div> </div><div>y</div>`, 1);
export default function App($$anchor) {
	function fn() {
		return 1;
	}
	var fragment = root();
	var div = $.first_child(fragment);
	var text = $.child(div, true);
	$.reset(div);
	var div_1 = $.sibling(div);
	$.template_effect(($0, $1) => {
		$.set_text(text, $1);
		$.set_attribute(div_1, "id", $0);
	}, [() => fn()], [() => "x"]);
	$.append($$anchor, fragment);
}
