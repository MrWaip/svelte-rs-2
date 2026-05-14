import * as $ from "svelte/internal/client";
import { timerStore } from "./store";
var root = $.from_html(`<button>start</button> <button>clear</button> <!>`, 1);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const $timerStore = () => $.store_get(timerStore, "$timerStore", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	let timerId = $.state(void 0);
	function clear() {
		if ($.get(timerId)) timerStore.clearTimer($.get(timerId));
	}
	function start() {
		$.set(timerId, timerStore.createTimer(60), true);
	}
	var fragment = root();
	var button = $.first_child(fragment);
	var button_1 = $.sibling(button, 2);
	var node = $.sibling(button_1, 2);
	{
		var consequent = ($$anchor) => {
			var text = $.text("active");
			$.append($$anchor, text);
		};
		$.if(node, ($$render) => {
			if ($.get(timerId) && $timerStore()[$.get(timerId)]) $$render(consequent);
		});
	}
	$.delegated("click", button, start);
	$.delegated("click", button_1, clear);
	$.append($$anchor, fragment);
	$.pop();
	$$cleanup();
}
$.delegate(["click"]);
