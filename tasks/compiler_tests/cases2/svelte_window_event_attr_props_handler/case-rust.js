import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.event("click", $.window, function(...$$args) {
		$$props.onClick?.apply(this, $$args);
	});
	$.event("custom-event", $.window, function(...$$args) {
		$$props.onCustom?.apply(this, $$args);
	});
}
