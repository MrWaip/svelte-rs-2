import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { object } = $$props;
	let num = 0;
	$.await($$renderer, object, () => {}, ({ [`k${num++}`]: v }) => {
		$$renderer.push(`<button>${$.escape(v)} ${$.escape(num)}</button>`);
	});
	$$renderer.push(`<!--]-->`);
}
