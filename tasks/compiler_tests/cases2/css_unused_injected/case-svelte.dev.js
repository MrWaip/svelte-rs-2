App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div class="used svelte-1n36she">used</div>`), App[$.FILENAME], [[18, 0]]);
const $$css = {
	hash: "svelte-1n36she",
	code: "\n	.used.svelte-1n36she {\n		color: red;\n	}\n\n	/* (unused) .unused {\n		color: blue;\n	}*/\n\n	.used.svelte-1n36she /* (unused) .unused-mixed*/ {\n		border: 1px solid;\n	}\n\n/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiKHVua25vd24pIiwic291cmNlcyI6WyIodW5rbm93bikiXSwic291cmNlc0NvbnRlbnQiOlsiPHN2ZWx0ZTpvcHRpb25zIGNzcz1cImluamVjdGVkXCIgLz5cblxuPHN0eWxlPlxuXHQudXNlZCB7XG5cdFx0Y29sb3I6IHJlZDtcblx0fVxuXG5cdC51bnVzZWQge1xuXHRcdGNvbG9yOiBibHVlO1xuXHR9XG5cblx0LnVzZWQsXG5cdC51bnVzZWQtbWl4ZWQge1xuXHRcdGJvcmRlcjogMXB4IHNvbGlkO1xuXHR9XG48L3N0eWxlPlxuXG48ZGl2IGNsYXNzPVwidXNlZFwiPnVzZWQ8L2Rpdj5cbiJdLCJuYW1lcyI6W10sIm1hcHBpbmdzIjoiO0FBR0EsQ0FBQyxvQkFBSyxDQUFDO0FBQ1AsRUFBRSxVQUFVO0FBQ1o7O0FBRUEsYUFBQztBQUNEO0FBQ0E7O0FBRUEsQ0FBQyxvQkFBSyxhQUNMLGVBQWEsQ0FBQztBQUNmLEVBQUUsaUJBQWlCO0FBQ25COyJ9 */"
};
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	$.append_styles($$anchor, $$css);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.append($$anchor, div);
	return $.pop($$exports);
}
