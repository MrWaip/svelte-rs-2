import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
var root = $.from_html(`<!> <button>x</button>`, 1);
export default function App($$anchor) {
	let refs = $.proxy([]);
	let items = $.proxy([{ id: 1 }, { id: 2 }]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 19, () => items, (item) => item.id, ($$anchor, item, i) => {
		var fragment_1 = root();
		var node_1 = $.first_child(fragment_1);
		$.bind_this(Comp(node_1, {}), ($$value, i) => refs[i] = $$value, (i) => refs?.[i], () => [$.get(i)]);
		var button = $.sibling(node_1, 2);
		$.delegated("click", button, () => refs[$.get(i)].foo());
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
