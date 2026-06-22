import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let elem = $.state(void 0);
	$.user_effect(() => {
		console.log($.get(elem));
	});
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.component(node, () => Inner, ($$anchor, $$component) => {
		$.bind_this($$component($$anchor, {}), ($$value) => $.set(elem, $$value), () => $.get(elem));
	});
	$.append($$anchor, fragment);
	$.pop();
}
