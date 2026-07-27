import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor) {
	let ratings = $.proxy([0, 1]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => ratings, $.index, ($$anchor, value, index) => {
		Child($$anchor, { onChange: (v) => ratings[index] = v });
	});
	$.append($$anchor, fragment);
}
