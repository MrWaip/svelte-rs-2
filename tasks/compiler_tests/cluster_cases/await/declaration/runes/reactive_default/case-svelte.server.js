import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { object } = $$props;
	let num = 0;
	function inc() {
		num++;
	}
	$$renderer.push(`<button>inc</button> `);
	$.await($$renderer, object, () => {}, ({ v = num }) => {
		$$renderer.push(`<button>${$.escape(v)} ${$.escape(num)}</button>`);
	});
	$$renderer.push(`<!--]-->`);
}
