App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { onMount } from "svelte";
import { writable } from "svelte/store";
import { fade, fly, slide } from "svelte/transition";
import { flip } from "svelte/animate";
import ChildComponent from "./Child.svelte";
const badge = $.wrap_snippet(App, function($$anchor, text = $.noop, variant = $.noop) {
	$.validate_snippet_args(...arguments);
	var span = root_2();
	let classes;
	var text_1 = $.child(span, true);
	$.reset(span);
	$.template_effect(() => {
		classes = $.set_class(span, 1, "badge svelte-13nvtxg", null, classes, {
			primary: $.strict_equals(variant(), "primary"),
			secondary: $.strict_equals(variant(), "secondary")
		});
		$.set_text(text_1, text());
	});
	$.append($$anchor, span);
});
const card = $.wrap_snippet(App, function($$anchor, heading = $.noop, body = $.noop) {
	$.validate_snippet_args(...arguments);
	var div = root_3();
	var h3 = $.child(div);
	var text_2 = $.child(h3, true);
	$.reset(h3);
	var p = $.sibling(h3, 2);
	var text_3 = $.child(p, true);
	$.reset(p);
	var node_1 = $.sibling(p, 2);
	$.add_svelte_meta(() => badge(node_1, () => "new", () => "primary"), "render", App, 174, 8);
	$.reset(div);
	$.template_effect(() => {
		$.set_text(text_2, heading());
		$.set_text(text_3, body());
	});
	$.append($$anchor, div);
});
export const BENCHMARK_KIND = "compiler";
export const MODULE_SCALE = 3;
export function moduleLabel(name) {
	return `${BENCHMARK_KIND}:${name}`;
}
var root_1 = $.add_locations($.from_html(`<meta name="description" content="Benchmark component" class="svelte-13nvtxg"/> <link rel="canonical" href="/benchmark" class="svelte-13nvtxg"/>`, 1), App[$.FILENAME], [[156, 4], [157, 4]]);
var root_2 = $.add_locations($.from_html(`<span> </span>`), App[$.FILENAME], [[165, 4]]);
var root_3 = $.add_locations($.from_html(`<div class="card svelte-13nvtxg"><h3 class="svelte-13nvtxg"> </h3> <p class="svelte-13nvtxg"> </p> <!></div>`), App[$.FILENAME], [[
	171,
	4,
	[[172, 8], [173, 8]]
]]);
var root_5 = $.add_locations($.from_html(`<span class="svelte-13nvtxg"> </span>`), App[$.FILENAME], [[182, 12]]);
var root_4 = $.add_locations($.from_html(`<section class="summary svelte-13nvtxg"><h4 class="svelte-13nvtxg"> </h4> <!></section>`), App[$.FILENAME], [[
	179,
	4,
	[[180, 8]]
]]);
var root_6 = $.add_locations($.from_html(`<span empty="" class="svelte-13nvtxg"> </span>`), App[$.FILENAME], [[216, 12]]);
var root_8 = $.add_locations($.from_html(`<h1 class="svelte-13nvtxg">Lorem ipsum dolor sit amet. Chunk 0.</h1>`), App[$.FILENAME], [[225, 16]]);
var root_10 = $.add_locations($.from_html(`<h2 class="svelte-13nvtxg">EMPTY</h2>`), App[$.FILENAME], [[231, 16]]);
var root_7 = $.add_locations($.from_html(`<div class="svelte-13nvtxg"><input class="svelte-13nvtxg"/></div> <!>`, 1), App[$.FILENAME], [[
	220,
	12,
	[[221, 16]]
]]);
var root_11 = $.add_locations($.from_html(`<p class="svelte-13nvtxg"> </p>`), App[$.FILENAME], [[237, 8]]);
var root_12 = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[242, 8]]);
var root_13 = $.add_locations($.from_html(`<span class="item-less svelte-13nvtxg">Repeated shell chunk 0</span>`), App[$.FILENAME], [[246, 8]]);
var root_14 = $.add_locations($.from_html(`<p class="svelte-13nvtxg"> </p>`), App[$.FILENAME], [[252, 8]]);
var root_15 = $.add_locations($.from_html(`<p class="svelte-13nvtxg"> </p>`), App[$.FILENAME], [[254, 8]]);
var root_16 = $.add_locations($.from_html(`<p class="svelte-13nvtxg">Loading chunk 0...</p>`), App[$.FILENAME], [[250, 8]]);
var root_17 = $.add_locations($.from_html(`<p class="svelte-13nvtxg"> </p>`), App[$.FILENAME], [[258, 8]]);
var root_19 = $.add_locations($.from_html(`<strong class="svelte-13nvtxg"> </strong>`), App[$.FILENAME], [[280, 8]]);
var root_20 = $.add_locations($.from_html(`<div slot="footer" class="svelte-13nvtxg"> </div>`), App[$.FILENAME], [[281, 8]]);
var root_21 = $.add_locations($.from_html(`<p class="svelte-13nvtxg"> </p>`), App[$.FILENAME], [[295, 12]]);
var root_22 = $.add_locations($.from_html(`<p class="svelte-13nvtxg"> </p>`), App[$.FILENAME], [[293, 8]]);
var root = $.add_locations($.from_html(`<div class="chunk-shell benchmark-reset benchmark-host svelte-13nvtxg" data-kind="chunk-0"> <p class="svelte-13nvtxg"> </p> <p class="svelte-13nvtxg"> </p> <!> <div>Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. <!></div> <!> <!> <!> <!> <!> <input class="svelte-13nvtxg"/> <textarea class="svelte-13nvtxg"></textarea> <select class="svelte-13nvtxg"><option class="svelte-13nvtxg">Zero</option><option class="svelte-13nvtxg">One</option></select> <input type="checkbox" class="svelte-13nvtxg"/> <input type="radio" class="svelte-13nvtxg"/> <div contenteditable="" class="svelte-13nvtxg">editable</div> <video class="svelte-13nvtxg"></video> <div class="svelte-13nvtxg">action target</div> <div class="svelte-13nvtxg">transition target</div> <div class="svelte-13nvtxg">in/out target</div> <!> <!> <!> <!> <!> <!> <button class="svelte-13nvtxg">Update store</button> <p class="svelte-13nvtxg"> </p> <!></div>`, 2), App[$.FILENAME], [[
	187,
	0,
	[
		[189, 4],
		[190, 4],
		[195, 4],
		[261, 4],
		[262, 4],
		[
			263,
			4,
			[[264, 8], [265, 8]]
		],
		[267, 4],
		[268, 4],
		[269, 4],
		[270, 4],
		[272, 4],
		[273, 4],
		[274, 4],
		[289, 4],
		[290, 4]
	]
]]);
const $$css = {
	hash: "svelte-13nvtxg",
	code: "\n    body {\n        margin: 0;\n        font-family: \"IBM Plex Sans\", sans-serif;\n        background: #f5f1e8;\n    }\n\n    .benchmark-host {\n        color: #3f2a18;\n    }\n\n    /* :global {*/\n        .benchmark-reset {\n            box-sizing: border-box;\n        }\n    /*}*/\n\n    @keyframes svelte-13nvtxg-pulse {\n        0% { opacity: 0.4; transform: scale(0.98); }\n        100% { opacity: 1; transform: scale(1); }\n    }\n\n    @keyframes marquee {\n        from { transform: translateX(0); }\n        to { transform: translateX(12px); }\n    }\n\n    .chunk-shell.svelte-13nvtxg {\n        padding: 16px;\n        margin: 12px 0;\n        border: 1px solid #d9c7ab;\n        background: linear-gradient(180deg, #fffdf9 0%, #f4ead9 100%);\n    }\n\n    .chunk-shell.svelte-13nvtxg :is(.badge:where(.svelte-13nvtxg), .card:where(.svelte-13nvtxg), .summary:where(.svelte-13nvtxg)) {\n        border-radius: 10px;\n    }\n\n    /* (unused) .chunk-shell.state .summary {\n        animation: pulse 180ms ease-out;\n    }*/\n\n    .summary.svelte-13nvtxg span:where(.svelte-13nvtxg) {\n        display: inline-block;\n        margin-right: 8px;\n    }\n\n    .item-less.svelte-13nvtxg {\n        color: #7a4f2a;\n    }\n\n    [data-index].svelte-13nvtxg {\n        color: var(--custom, #5c4634);\n    }\n\n/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiKHVua25vd24pIiwic291cmNlcyI6WyIodW5rbm93bikiXSwic291cmNlc0NvbnRlbnQiOlsiPHNjcmlwdCBtb2R1bGU+XG4gICAgZXhwb3J0IGNvbnN0IEJFTkNITUFSS19LSU5EID0gXCJjb21waWxlclwiO1xuICAgIGV4cG9ydCBjb25zdCBNT0RVTEVfU0NBTEUgPSAzO1xuXG4gICAgZXhwb3J0IGZ1bmN0aW9uIG1vZHVsZUxhYmVsKG5hbWUpIHtcbiAgICAgICAgcmV0dXJuIGAke0JFTkNITUFSS19LSU5EfToke25hbWV9YDtcbiAgICB9XG48L3NjcmlwdD5cblxuPHNjcmlwdD5cbiAgICBpbXBvcnQgeyBvbk1vdW50IH0gZnJvbSBcInN2ZWx0ZVwiO1xuICAgIGltcG9ydCB7IHdyaXRhYmxlIH0gZnJvbSBcInN2ZWx0ZS9zdG9yZVwiO1xuICAgIGltcG9ydCB7IGZhZGUsIGZseSwgc2xpZGUgfSBmcm9tIFwic3ZlbHRlL3RyYW5zaXRpb25cIjtcbiAgICBpbXBvcnQgeyBmbGlwIH0gZnJvbSBcInN2ZWx0ZS9hbmltYXRlXCI7XG4gICAgaW1wb3J0IENoaWxkQ29tcG9uZW50IGZyb20gXCIuL0NoaWxkLnN2ZWx0ZVwiO1xuXG4gICAgbGV0IHtcbiAgICAgICAgdGl0bGUgPSBcIkRlZmF1bHQgVGl0bGVcIixcbiAgICAgICAgY291bnQgPSAwLFxuICAgICAgICBpdGVtcyA9IFtdLFxuICAgICAgICBjb25maWcgPSAkYmluZGFibGUoe30pLFxuICAgICAgICBtdWx0aXBsaWVyID0gMixcbiAgICAgICAgdmlzaWJsZSA9ICRiaW5kYWJsZShmYWxzZSksXG4gICAgICAgIC4uLnJlc3RcbiAgICB9ID0gJHByb3BzKCk7XG5cbiAgICBjb25zdCBwcm9wc0lkID0gJHByb3BzLmlkKCk7XG5cbiAgICBsZXQgc3RhdGUgPSAkc3RhdGUoXCJcIik7XG4gICAgbGV0IGNvdW50ZXIgPSAkc3RhdGUoMCk7XG4gICAgbGV0IHJhd0RhdGEgPSAkc3RhdGUucmF3KHsgeDogMSwgeTogMiB9KTtcbiAgICBsZXQgY2hlY2tlZCA9ICRzdGF0ZShmYWxzZSk7XG4gICAgbGV0IGdyb3VwID0gJHN0YXRlKFtdKTtcbiAgICBsZXQgdm9sdW1lID0gJHN0YXRlKDAuNSk7XG4gICAgbGV0IHNlbGVjdGVkID0gJHN0YXRlKFwib3B0LTBcIik7XG4gICAgbGV0IGlucHV0RWw7XG4gICAgbGV0IGNvbXBvbmVudFJlZjtcbiAgICBsZXQgZHluYW1pY0VsO1xuXG4gICAgbGV0IG1ldHJpY3MgPSB3cml0YWJsZShbMSwgMiwgM10pO1xuICAgIGxldCBsYWJlbFN0b3JlID0gd3JpdGFibGUoXCJyZWFkeVwiKTtcblxuICAgIC8qKiBAdHlwZSB7RnVuY3Rpb24gfCB1bmRlZmluZWR9ICovXG4gICAgbGV0IHNob3c7XG5cbiAgICBjb3VudGVyID0gMTA7XG5cbiAgICBsZXQgZG91YmxlZCA9ICRkZXJpdmVkKGNvdW50ICogbXVsdGlwbGllcik7XG4gICAgbGV0IGNvbXB1dGVkID0gJGRlcml2ZWQuYnkoKCkgPT4ge1xuICAgICAgICByZXR1cm4gaXRlbXMubGVuZ3RoICogbXVsdGlwbGllciArIGNvdW50ZXI7XG4gICAgfSk7XG4gICAgbGV0IG1vZHVsZVN1bW1hcnkgPSAkZGVyaXZlZChtb2R1bGVMYWJlbCh0aXRsZSkgKyBcIjpcIiArIE1PRFVMRV9TQ0FMRSk7XG4gICAgbGV0IHN0b3JlU3VtbWFyeSA9ICRkZXJpdmVkKCRtZXRyaWNzLmxlbmd0aCArIFwiOlwiICsgJGxhYmVsU3RvcmUpO1xuICAgIGxldCBzbmFwc2hvdCA9ICRzdGF0ZS5zbmFwc2hvdChyYXdEYXRhKTtcblxuICAgICRlZmZlY3QoKCkgPT4ge1xuICAgICAgICBjb25zb2xlLmxvZyhcIlRpdGxlOlwiLCB0aXRsZSwgXCJDb3VudDpcIiwgY291bnQpO1xuICAgIH0pO1xuXG4gICAgJGVmZmVjdC5wcmUoKCkgPT4ge1xuICAgICAgICBjb25zb2xlLmxvZyhcIlByZSBlZmZlY3Q6XCIsIGNvdW50ZXIpO1xuICAgIH0pO1xuXG4gICAgbGV0IHRyYWNraW5nID0gJGVmZmVjdC50cmFja2luZygpO1xuXG4gICAgJGluc3BlY3QoY291bnRlciwgZG91YmxlZCk7XG5cbiAgICBleHBvcnQgY29uc3QgQVBQX1ZFUlNJT04gPSBcIjEuMC4wXCI7XG5cbiAgICBleHBvcnQgZnVuY3Rpb24gZm9ybWF0VGl0bGUocHJlZml4KSB7XG4gICAgICAgIHJldHVybiBwcmVmaXggKyBcIjogXCIgKyB0aXRsZTtcbiAgICB9XG5cbiAgICBmdW5jdGlvbiBhZGRNZXRyaWMoKSB7XG4gICAgICAgICRtZXRyaWNzID0gWy4uLiRtZXRyaWNzLCBjb3VudGVyXTtcbiAgICAgICAgJGxhYmVsU3RvcmUgPSB0aXRsZTtcbiAgICB9XG5cbiAgICBmdW5jdGlvbiBhY3Rpb24obm9kZSwgYXJnKSB7XG4gICAgICAgIHJldHVybiB7IGRlc3Ryb3koKSB7fSB9O1xuICAgIH1cblxuICAgIGZ1bmN0aW9uIGhhbmRsZUNsaWNrKGUpIHtcbiAgICAgICAgY291bnRlcisrO1xuICAgIH1cblxuICAgIGZ1bmN0aW9uIGdldEhhbmRsZXIoKSB7XG4gICAgICAgIHJldHVybiBoYW5kbGVDbGljaztcbiAgICB9XG5cbiAgICBmdW5jdGlvbiBoYW5kbGVFcnJvcihlcnJvcikge1xuICAgICAgICBjb25zb2xlLmVycm9yKGVycm9yKTtcbiAgICB9XG5cbiAgICBsZXQgcHJvbWlzZSA9IFByb21pc2UucmVzb2x2ZSg0Mik7XG48L3NjcmlwdD5cblxuPHN0eWxlPlxuICAgIDpnbG9iYWwoYm9keSkge1xuICAgICAgICBtYXJnaW46IDA7XG4gICAgICAgIGZvbnQtZmFtaWx5OiBcIklCTSBQbGV4IFNhbnNcIiwgc2Fucy1zZXJpZjtcbiAgICAgICAgYmFja2dyb3VuZDogI2Y1ZjFlODtcbiAgICB9XG5cbiAgICA6Z2xvYmFsKC5iZW5jaG1hcmstaG9zdCkge1xuICAgICAgICBjb2xvcjogIzNmMmExODtcbiAgICB9XG5cbiAgICA6Z2xvYmFsIHtcbiAgICAgICAgLmJlbmNobWFyay1yZXNldCB7XG4gICAgICAgICAgICBib3gtc2l6aW5nOiBib3JkZXItYm94O1xuICAgICAgICB9XG4gICAgfVxuXG4gICAgQGtleWZyYW1lcyBwdWxzZSB7XG4gICAgICAgIDAlIHsgb3BhY2l0eTogMC40OyB0cmFuc2Zvcm06IHNjYWxlKDAuOTgpOyB9XG4gICAgICAgIDEwMCUgeyBvcGFjaXR5OiAxOyB0cmFuc2Zvcm06IHNjYWxlKDEpOyB9XG4gICAgfVxuXG4gICAgQGtleWZyYW1lcyAtZ2xvYmFsLW1hcnF1ZWUge1xuICAgICAgICBmcm9tIHsgdHJhbnNmb3JtOiB0cmFuc2xhdGVYKDApOyB9XG4gICAgICAgIHRvIHsgdHJhbnNmb3JtOiB0cmFuc2xhdGVYKDEycHgpOyB9XG4gICAgfVxuXG4gICAgLmNodW5rLXNoZWxsIHtcbiAgICAgICAgcGFkZGluZzogMTZweDtcbiAgICAgICAgbWFyZ2luOiAxMnB4IDA7XG4gICAgICAgIGJvcmRlcjogMXB4IHNvbGlkICNkOWM3YWI7XG4gICAgICAgIGJhY2tncm91bmQ6IGxpbmVhci1ncmFkaWVudCgxODBkZWcsICNmZmZkZjkgMCUsICNmNGVhZDkgMTAwJSk7XG4gICAgfVxuXG4gICAgLmNodW5rLXNoZWxsIDppcyguYmFkZ2UsIC5jYXJkLCAuc3VtbWFyeSkge1xuICAgICAgICBib3JkZXItcmFkaXVzOiAxMHB4O1xuICAgIH1cblxuICAgIC5jaHVuay1zaGVsbC5zdGF0ZSAuc3VtbWFyeSB7XG4gICAgICAgIGFuaW1hdGlvbjogcHVsc2UgMTgwbXMgZWFzZS1vdXQ7XG4gICAgfVxuXG4gICAgLnN1bW1hcnkgc3BhbiB7XG4gICAgICAgIGRpc3BsYXk6IGlubGluZS1ibG9jaztcbiAgICAgICAgbWFyZ2luLXJpZ2h0OiA4cHg7XG4gICAgfVxuXG4gICAgLml0ZW0tbGVzcyB7XG4gICAgICAgIGNvbG9yOiAjN2E0ZjJhO1xuICAgIH1cblxuICAgIFtkYXRhLWluZGV4XSB7XG4gICAgICAgIGNvbG9yOiB2YXIoLS1jdXN0b20sICM1YzQ2MzQpO1xuICAgIH1cbjwvc3R5bGU+XG5cbjxzdmVsdGU6aGVhZD5cbiAgICA8dGl0bGU+e3RpdGxlfSAtIEJlbmNobWFyazwvdGl0bGU+XG4gICAgPG1ldGEgbmFtZT1cImRlc2NyaXB0aW9uXCIgY29udGVudD1cIkJlbmNobWFyayBjb21wb25lbnRcIj5cbiAgICA8bGluayByZWw9XCJjYW5vbmljYWxcIiBocmVmPVwiL2JlbmNobWFya1wiPlxuPC9zdmVsdGU6aGVhZD5cblxuPHN2ZWx0ZTp3aW5kb3cgb25zY3JvbGw9e2hhbmRsZUNsaWNrfSAvPlxuPHN2ZWx0ZTpkb2N1bWVudCBvbnZpc2liaWxpdHljaGFuZ2U9e2hhbmRsZUNsaWNrfSAvPlxuPHN2ZWx0ZTpib2R5IG9ubW91c2VlbnRlcj17aGFuZGxlQ2xpY2t9IHVzZTphY3Rpb249e3N0YXRlfSAvPlxuXG57I3NuaXBwZXQgYmFkZ2UodGV4dCwgdmFyaWFudCl9XG4gICAgPHNwYW4gY2xhc3M9XCJiYWRnZVwiIGNsYXNzOnByaW1hcnk9e3ZhcmlhbnQgPT09IFwicHJpbWFyeVwifSBjbGFzczpzZWNvbmRhcnk9e3ZhcmlhbnQgPT09IFwic2Vjb25kYXJ5XCJ9PlxuICAgICAgICB7dGV4dH1cbiAgICA8L3NwYW4+XG57L3NuaXBwZXR9XG5cbnsjc25pcHBldCBjYXJkKGhlYWRpbmcsIGJvZHkpfVxuICAgIDxkaXYgY2xhc3M9XCJjYXJkXCI+XG4gICAgICAgIDxoMz57aGVhZGluZ308L2gzPlxuICAgICAgICA8cD57Ym9keX08L3A+XG4gICAgICAgIHtAcmVuZGVyIGJhZGdlKFwibmV3XCIsIFwicHJpbWFyeVwiKX1cbiAgICA8L2Rpdj5cbnsvc25pcHBldH1cblxueyNzbmlwcGV0IG1ldHJpY1N1bW1hcnkoeyBsYWJlbCwgdmFsdWVzID0gW2NvdW50ZXJdLCBtZXRhOiB7IGlkID0gcHJvcHNJZCB9ID0ge30gfSl9XG4gICAgPHNlY3Rpb24gY2xhc3M9XCJzdW1tYXJ5XCIgZGF0YS1pZD17aWR9PlxuICAgICAgICA8aDQ+e2xhYmVsfTwvaDQ+XG4gICAgICAgIHsjZWFjaCB2YWx1ZXMgYXMgdmFsdWUsIGluZGV4fVxuICAgICAgICAgICAgPHNwYW4+e2luZGV4fToge3ZhbHVlfTwvc3Bhbj5cbiAgICAgICAgey9lYWNofVxuICAgIDwvc2VjdGlvbj5cbnsvc25pcHBldH1cblxuPGRpdiBjbGFzcz1cImNodW5rLXNoZWxsIGJlbmNobWFyay1yZXNldCBiZW5jaG1hcmstaG9zdFwiIGRhdGEta2luZD1cImNodW5rLTBcIj5cbiAgICBDaHVuayAwOiBMb3JlbSB7c3RhdGV9ICsge3N0YXRlfSA9IElwc3VtO1xuICAgIDxwPlByb3BzOiB0aXRsZT17dGl0bGV9LCBjb3VudD17Y291bnR9LCBkb3VibGVkPXtkb3VibGVkfSwgY29tcHV0ZWQ9e2NvbXB1dGVkfTwvcD5cbiAgICA8cD5Nb2R1bGU6IHttb2R1bGVTdW1tYXJ5fSB8IFN0b3JlOiB7c3RvcmVTdW1tYXJ5fSB8IExhYmVsOiB7JGxhYmVsU3RvcmV9PC9wPlxuXG4gICAge0BodG1sIFwiPGI+cmF3IGh0bWwgY2h1bmsgMDwvYj5cIn1cbiAgICB7QGRlYnVnIGNvdW50ZXIsIHN0YXRlfVxuXG4gICAgPGRpdlxuICAgICAgICBjbGFzczpzdGF0ZVxuICAgICAgICBjbGFzczpzdGF0aWNseT17dHJ1ZX1cbiAgICAgICAgY2xhc3M6aW52aW5zaWJsZVxuICAgICAgICBjbGFzczpyZWFjdGl2ZT17Y291bnRlcn1cbiAgICAgICAgY2xhc3M9e3sgYWN0aXZlOiBjaGVja2VkLCBiaWc6IGNvdW50ZXIgPiAxMCB9fVxuICAgICAgICBzdHlsZTpjb2xvcj17c3RhdGV9XG4gICAgICAgIHN0eWxlOmZvbnQtc2l6ZT1cIjE0cHhcIlxuICAgICAgICBzdHlsZTpvcGFjaXR5PXtjb3VudGVyIC8gMTAwfVxuICAgICAgICBzdHlsZTotLWN1c3RvbT1cInZhbHVlLTBcIlxuICAgICAgICBvbmNsaWNrPXtoYW5kbGVDbGlja31cbiAgICAgICAgb25zY3JvbGw9e2hhbmRsZUNsaWNrfVxuICAgICAgICBvbmNsaWNrY2FwdHVyZT17aGFuZGxlQ2xpY2t9XG4gICAgICAgIG9uZm9jdXM9e2dldEhhbmRsZXIoKX1cbiAgICAgICAgYmluZDp0aGlzPXtkeW5hbWljRWx9XG4gICAgPlxuICAgICAgICBMb3JlbSBpcHN1bSBkb2xvciBzaXQgYW1ldCwgY29uc2VjdGV0dXIgYWRpcGlzY2luZyBlbGl0LCBzZWQgZG8gZWl1c21vZFxuICAgICAgICB0ZW1wb3IgaW5jaWRpZHVudCB1dCBsYWJvcmUgZXQgZG9sb3JlIG1hZ25hIGFsaXF1YS5cblxuICAgICAgICB7I2lmIHN0YXRlfVxuICAgICAgICAgICAge0Bjb25zdCBsb2NhbExlbiA9IHN0YXRlLmxlbmd0aH1cbiAgICAgICAgICAgIDxzcGFuIHRpdGxlPVwie3RpdGxlfToge2RvdWJsZWR9XCIgZW1wdHkge3N0YXRlfSB7Y291bnRlcn0gY291bnQ9e2NvdW50fT5cbiAgICAgICAgICAgICAgICBEdWlzIGF1dGUgaXJ1cmUgZG9sb3I6IHtsb2NhbExlbn0uIENodW5rIDAuXG4gICAgICAgICAgICA8L3NwYW4+XG4gICAgICAgIHs6ZWxzZX1cbiAgICAgICAgICAgIDxkaXY+XG4gICAgICAgICAgICAgICAgPGlucHV0IHt0aXRsZX0ge3N0YXRlfSB2YWx1ZT17Y291bnR9IC8+XG4gICAgICAgICAgICA8L2Rpdj5cblxuICAgICAgICAgICAgeyNpZiBjb3VudGVyID4gMzB9XG4gICAgICAgICAgICAgICAgPGgxIHtzdGF0ZX0+XG4gICAgICAgICAgICAgICAgICAgIExvcmVtIGlwc3VtIGRvbG9yIHNpdCBhbWV0LiBDaHVuayAwLlxuICAgICAgICAgICAgICAgIDwvaDE+XG4gICAgICAgICAgICB7OmVsc2UgaWYgY291bnRlciA9PSAxMDB9XG4gICAgICAgICAgICAgICAgTG9yZW0gaXBzdW0gZG9sb3Igc2l0IGFtZXQuIENodW5rIDAuXG4gICAgICAgICAgICB7OmVsc2V9XG4gICAgICAgICAgICAgICAgPGgyPkVNUFRZPC9oMj5cbiAgICAgICAgICAgIHsvaWZ9XG4gICAgICAgIHsvaWZ9XG4gICAgPC9kaXY+XG5cbiAgICB7I2tleSBjb3VudGVyfVxuICAgICAgICA8cCB0cmFuc2l0aW9uOnNsaWRlPktleWVkIGNvbnRlbnQgY2h1bmsgMDoge2NvdW50ZXJ9PC9wPlxuICAgIHsva2V5fVxuXG4gICAgeyNlYWNoIGl0ZW1zIGFzIGl0ZW0sIGlkeCAoaXRlbS5pZCl9XG4gICAgICAgIHtAY29uc3QgaXRlbUxhYmVsID0gYCR7aWR4fToke2l0ZW0ubmFtZX1gfVxuICAgICAgICA8cCB7Li4ucmVzdH0gZGF0YS1pbmRleD1cImNodW5rLTAte2lkeH1cIiBhbmltYXRlOmZsaXA+e2l0ZW1MYWJlbH08L3A+XG4gICAgey9lYWNofVxuXG4gICAgeyNlYWNoIGl0ZW1zfVxuICAgICAgICA8c3BhbiBjbGFzcz1cIml0ZW0tbGVzc1wiPlJlcGVhdGVkIHNoZWxsIGNodW5rIDA8L3NwYW4+XG4gICAgey9lYWNofVxuXG4gICAgeyNhd2FpdCBwcm9taXNlfVxuICAgICAgICA8cD5Mb2FkaW5nIGNodW5rIDAuLi48L3A+XG4gICAgezp0aGVuIHZhbHVlfVxuICAgICAgICA8cD5SZXNvbHZlZDoge3ZhbHVlfTwvcD5cbiAgICB7OmNhdGNoIGVycm9yfVxuICAgICAgICA8cD5FcnJvcjoge2Vycm9yLm1lc3NhZ2V9PC9wPlxuICAgIHsvYXdhaXR9XG5cbiAgICB7I2F3YWl0IHByb21pc2UgdGhlbiBxdWlja1ZhbHVlfVxuICAgICAgICA8cD5RdWljayByZXNvbHZlZDoge3F1aWNrVmFsdWV9PC9wPlxuICAgIHsvYXdhaXR9XG5cbiAgICA8aW5wdXQgYmluZDp2YWx1ZT17c3RhdGV9IC8+XG4gICAgPHRleHRhcmVhIGJpbmQ6dmFsdWU9e3N0YXRlfSAvPlxuICAgIDxzZWxlY3QgYmluZDp2YWx1ZT17c2VsZWN0ZWR9PlxuICAgICAgICA8b3B0aW9uIHZhbHVlPVwib3B0LTBcIj5aZXJvPC9vcHRpb24+XG4gICAgICAgIDxvcHRpb24gdmFsdWU9XCJvcHQtMVwiPk9uZTwvb3B0aW9uPlxuICAgIDwvc2VsZWN0PlxuICAgIDxpbnB1dCB0eXBlPVwiY2hlY2tib3hcIiBiaW5kOmNoZWNrZWQ9e2NoZWNrZWR9IC8+XG4gICAgPGlucHV0IHR5cGU9XCJyYWRpb1wiIGJpbmQ6Z3JvdXA9e2dyb3VwfSB2YWx1ZT1cIm9wdC0wXCIgLz5cbiAgICA8ZGl2IGJpbmQ6dGhpcz17aW5wdXRFbH0gYmluZDpjbGllbnRXaWR0aD17Y291bnRlcn0gY29udGVudGVkaXRhYmxlIGJpbmQ6aW5uZXJIVE1MPXtzdGF0ZX0+ZWRpdGFibGU8L2Rpdj5cbiAgICA8dmlkZW8gYmluZDp2b2x1bWU9e3ZvbHVtZX0gYmluZDpwYXVzZWQ9e2NoZWNrZWR9PjwvdmlkZW8+XG5cbiAgICA8ZGl2IHVzZTphY3Rpb249e3N0YXRlfT5hY3Rpb24gdGFyZ2V0PC9kaXY+XG4gICAgPGRpdiB0cmFuc2l0aW9uOmZhZGU+dHJhbnNpdGlvbiB0YXJnZXQ8L2Rpdj5cbiAgICA8ZGl2IGluOmZseT17eyB5OiAyMDAgfX0gb3V0OmZhZGU+aW4vb3V0IHRhcmdldDwvZGl2PlxuICAgIDxzdmVsdGU6ZWxlbWVudCB0aGlzPXtzdGF0ZSA/IFwiZGl2XCIgOiBcInNwYW5cIn0gY2xhc3M9XCJkeW5hbWljLTBcIj5cbiAgICAgICAgRHluYW1pYyBlbGVtZW50IGNodW5rIDA6IHt0aXRsZX1cbiAgICA8L3N2ZWx0ZTplbGVtZW50PlxuXG4gICAgPENoaWxkQ29tcG9uZW50IGJpbmQ6dGhpcz17Y29tcG9uZW50UmVmfSB0aXRsZT17dGl0bGV9IG9uY2xpY2s9e2dldEhhbmRsZXIoKX0+XG4gICAgICAgIDxzdHJvbmc+SW5saW5lIGNoaWxkIGNodW5rIDA6IHt0aXRsZX08L3N0cm9uZz5cbiAgICAgICAgPGRpdiBzbG90PVwiZm9vdGVyXCI+Rm9vdGVyIGNodW5rIDA6IHtjb3VudGVyfTwvZGl2PlxuICAgIDwvQ2hpbGRDb21wb25lbnQ+XG5cbiAgICB7QHJlbmRlciBiYWRnZShcImNodW5rLTBcIiwgXCJzZWNvbmRhcnlcIil9XG4gICAge0ByZW5kZXIgY2FyZCh0aXRsZSwgXCJDb250ZW50IGZvciBjaHVuayAwXCIpfVxuICAgIHtAcmVuZGVyIG1ldHJpY1N1bW1hcnkoeyBsYWJlbDogdGl0bGUsIHZhbHVlczogW2NvdW50LCBkb3VibGVkLCBjb3VudGVyXSwgbWV0YTogeyBpZDogcHJvcHNJZCB9IH0pfVxuICAgIHtAcmVuZGVyIHNob3c/LigpfVxuXG4gICAgPGJ1dHRvbiBvbmNsaWNrPXthZGRNZXRyaWN9PlVwZGF0ZSBzdG9yZTwvYnV0dG9uPlxuICAgIDxwPk1ldHJpYyBjb3VudDogeyRtZXRyaWNzLmxlbmd0aH08L3A+XG5cbiAgICA8c3ZlbHRlOmJvdW5kYXJ5IG9uZXJyb3I9e2hhbmRsZUVycm9yfT5cbiAgICAgICAgPHA+Qm91bmRhcnkgY2h1bmsgMDoge3RpdGxlfTwvcD5cbiAgICAgICAgeyNzbmlwcGV0IGZhaWxlZChlcnJvcil9XG4gICAgICAgICAgICA8cD5FcnJvciBpbiBjaHVuayAwOiB7ZXJyb3IubWVzc2FnZX08L3A+XG4gICAgICAgIHsvc25pcHBldH1cbiAgICA8L3N2ZWx0ZTpib3VuZGFyeT5cbjwvZGl2PlxuIl0sIm5hbWVzIjpbXSwibWFwcGluZ3MiOiI7QUFrR0EsSUFBWSxJQUFLLENBQUM7QUFDbEIsUUFBUSxTQUFTO0FBQ2pCLFFBQVEsd0NBQXdDO0FBQ2hELFFBQVEsbUJBQW1CO0FBQzNCOztBQUVBLElBQVksZUFBZ0IsQ0FBQztBQUM3QixRQUFRLGNBQWM7QUFDdEI7O0FBRUEsT0FBSSxRQUFRO0FBQ1osUUFBUSxnQkFBZ0IsQ0FBQztBQUN6QixZQUFZLHNCQUFzQjtBQUNsQztBQUNBLE1BQUk7O0FBRUosSUFBSSwwQkFBVztBQUNmO0FBQ0E7QUFDQTs7QUFFQSxJQUFJLFdBQW1CO0FBQ3ZCO0FBQ0E7QUFDQTs7QUFFQSxJQUFJLDJCQUFZLENBQUM7QUFDakIsUUFBUSxhQUFhO0FBQ3JCLFFBQVEsY0FBYztBQUN0QixRQUFRLHlCQUF5QjtBQUNqQyxRQUFRLDZEQUE2RDtBQUNyRTs7QUFFQSxJQUFJLDJCQUFZLENBQUMsSUFBSSw2QkFBTSxFQUFFLDRCQUFLLEVBQUUsK0JBQVEsQ0FBQyxDQUFDO0FBQzlDLFFBQVEsbUJBQW1CO0FBQzNCOztBQUVBLGdCQUFJO0FBQ0o7QUFDQTs7QUFFQSxJQUFJLHVCQUFRLENBQUMsMkJBQUksQ0FBQztBQUNsQixRQUFRLHFCQUFxQjtBQUM3QixRQUFRLGlCQUFpQjtBQUN6Qjs7QUFFQSxJQUFJLHlCQUFVLENBQUM7QUFDZixRQUFRLGNBQWM7QUFDdEI7O0FBRUEsSUFBSSwyQkFBWSxDQUFDO0FBQ2pCLFFBQVEsNkJBQTZCO0FBQ3JDOyJ9 */"
};
export default function App($$anchor, $$props) {
	const propsId = $.props_id();
	$.check_target(new.target);
	$.push($$props, true, App);
	$.append_styles($$anchor, $$css);
	const $metrics = () => ($.validate_store(metrics, "metrics"), $.store_get(metrics, "$metrics", $$stores));
	const $labelStore = () => ($.validate_store(labelStore, "labelStore"), $.store_get(labelStore, "$labelStore", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const binding_group = [];
	const metricSummary = $.wrap_snippet(App, function($$anchor, $$arg0) {
		$.validate_snippet_args(...arguments);
		let label = () => $$arg0?.().label;
		label();
		let values = $.derived_safe_equal(() => $.fallback($$arg0?.().values, () => [$.get(counter)], true));
		$.get(values);
		let id = $.derived_safe_equal(() => $.fallback($.fallback($$arg0?.().meta, () => ({}), true).id, propsId));
		$.get(id);
		var section = root_4();
		var h4 = $.child(section);
		var text_4 = $.child(h4, true);
		$.reset(h4);
		var node_2 = $.sibling(h4, 2);
		$.add_svelte_meta(() => $.each(node_2, 17, () => $.get(values), $.index, ($$anchor, value, index) => {
			var span_1 = root_5();
			var text_5 = $.child(span_1);
			$.reset(span_1);
			$.template_effect(() => $.set_text(text_5, `${index}: ${$.get(value) ?? ""}`));
			$.append($$anchor, span_1);
		}), "each", App, 181, 8);
		$.reset(section);
		$.template_effect(() => {
			$.set_attribute(section, "data-id", $.get(id));
			$.set_text(text_4, label());
		});
		$.append($$anchor, section);
	});
	let title = $.prop($$props, "title", 7, "Default Title"), count = $.prop($$props, "count", 7, 0), items = $.prop($$props, "items", 23, () => []), config = $.prop($$props, "config", 31, () => $.tag_proxy($.proxy({}), "config")), multiplier = $.prop($$props, "multiplier", 7, 2), visible = $.prop($$props, "visible", 15, false), rest = $.rest_props($$props, [
		"$$slots",
		"$$events",
		"$$legacy",
		"$$host",
		"title",
		"count",
		"items",
		"config",
		"multiplier",
		"visible"
	], "rest");
	let state = $.tag($.state(""), "state");
	let counter = $.tag($.state(0), "counter");
	let rawData = $.tag($.state({
		x: 1,
		y: 2
	}), "rawData");
	let checked = $.tag($.state(false), "checked");
	let group = $.tag($.state($.proxy([])), "group");
	let volume = $.tag($.state(.5), "volume");
	let selected = $.tag($.state("opt-0"), "selected");
	let inputEl;
	let componentRef;
	let dynamicEl;
	let metrics = writable([
		1,
		2,
		3
	]);
	let labelStore = writable("ready");
	let show;
	$.set(counter, 10);
	let doubled = $.tag($.derived(() => count() * multiplier()), "doubled");
	let computed = $.tag($.derived(() => {
		return items().length * multiplier() + $.get(counter);
	}), "computed");
	let moduleSummary = $.tag($.derived(() => moduleLabel(title()) + ":" + MODULE_SCALE), "moduleSummary");
	let storeSummary = $.tag($.derived(() => $metrics().length + ":" + $labelStore()), "storeSummary");
	let snapshot = $.snapshot($.get(rawData));
	$.user_effect(() => {
		console.log(...$.log_if_contains_state("log", "Title:", title(), "Count:", count()));
	});
	$.user_pre_effect(() => {
		console.log(...$.log_if_contains_state("log", "Pre effect:", $.get(counter)));
	});
	let tracking = $.effect_tracking();
	$.inspect(() => [$.get(counter), $.get(doubled)], (...$$args) => console.log(...$$args), true);
	const APP_VERSION = "1.0.0";
	function formatTitle(prefix) {
		return prefix + ": " + title();
	}
	function addMetric() {
		$.store_set(metrics, [...$metrics(), $.get(counter)]);
		$.store_set(labelStore, title());
	}
	function action(node, arg) {
		return { destroy() {} };
	}
	function handleClick(e) {
		$.update(counter);
	}
	function getHandler() {
		return handleClick;
	}
	function handleError(error) {
		console.error(...$.log_if_contains_state("error", error));
	}
	let promise = Promise.resolve(42);
	var $$exports = {
		...$.legacy_api(),
		get APP_VERSION() {
			return APP_VERSION;
		},
		get formatTitle() {
			return formatTitle;
		},
		get title() {
			return title();
		},
		set title($$value = "Default Title") {
			title($$value);
			$.flush();
		},
		get count() {
			return count();
		},
		set count($$value = 0) {
			count($$value);
			$.flush();
		},
		get items() {
			return items();
		},
		set items($$value = []) {
			items($$value);
			$.flush();
		},
		get config() {
			return config();
		},
		set config($$value = {}) {
			config($$value);
			$.flush();
		},
		get multiplier() {
			return multiplier();
		},
		set multiplier($$value = 2) {
			multiplier($$value);
			$.flush();
		},
		get visible() {
			return visible();
		},
		set visible($$value = false) {
			visible($$value);
			$.flush();
		}
	};
	var div_1 = root();
	$.head("q2w0q4", ($$anchor) => {
		var fragment = root_1();
		$.next(2);
		$.deferred_template_effect(() => {
			$.document.title = `${title() ?? ""} - Benchmark`;
		});
		$.append($$anchor, fragment);
	});
	$.event("scroll", $.window, handleClick);
	$.event("visibilitychange", $.document, handleClick);
	$.event("mouseenter", $.document.body, handleClick);
	$.action($.document.body, ($$node, $$action_arg) => action?.($$node, $$action_arg), () => $.get(state));
	$.template_effect(() => {
		console.log({
			counter: $.snapshot($.get(counter)),
			state: $.snapshot($.get(state))
		});
		debugger;
	});
	var text_6 = $.child(div_1);
	var p_1 = $.sibling(text_6);
	var text_7 = $.child(p_1);
	$.reset(p_1);
	var p_2 = $.sibling(p_1, 2);
	var text_8 = $.child(p_2);
	$.reset(p_2);
	var node_3 = $.sibling(p_2, 2);
	$.html(node_3, () => "<b>raw html chunk 0</b>");
	var div_2 = $.sibling(node_3, 2);
	let classes_1;
	var event_handler = $.derived(getHandler);
	let styles;
	var node_4 = $.sibling($.child(div_2));
	{
		var consequent = ($$anchor) => {
			const localLen = $.tag($.derived(() => $.get(state).length), "localLen");
			$.get(localLen);
			var span_2 = root_6();
			var text_9 = $.child(span_2);
			$.reset(span_2);
			$.template_effect(() => {
				$.set_attribute(span_2, "title", `${title() ?? ""}: ${$.get(doubled) ?? ""}`);
				$.set_attribute(span_2, "state", $.get(state));
				$.set_attribute(span_2, "counter", $.get(counter));
				$.set_attribute(span_2, "count", count());
				$.set_text(text_9, `Duis aute irure dolor: ${$.get(localLen) ?? ""}. Chunk 0.`);
			});
			$.append($$anchor, span_2);
		};
		var alternate_1 = ($$anchor) => {
			var fragment_1 = root_7();
			var div_3 = $.first_child(fragment_1);
			var input = $.child(div_3);
			$.remove_input_defaults(input);
			$.reset(div_3);
			var node_5 = $.sibling(div_3, 2);
			{
				var consequent_1 = ($$anchor) => {
					var h1 = root_8();
					$.template_effect(() => $.set_attribute(h1, "state", $.get(state)));
					$.append($$anchor, h1);
				};
				var consequent_2 = ($$anchor) => {
					var text_10 = $.text("Lorem ipsum dolor sit amet. Chunk 0.");
					$.append($$anchor, text_10);
				};
				var alternate = ($$anchor) => {
					var h2 = root_10();
					$.append($$anchor, h2);
				};
				$.add_svelte_meta(() => $.if(node_5, ($$render) => {
					if ($.get(counter) > 30) $$render(consequent_1);
					else if ($.equals($.get(counter), 100)) $$render(consequent_2, 1);
					else $$render(alternate, -1);
				}), "if", App, 224, 12);
			}
			$.template_effect(() => {
				$.set_attribute(input, "title", title());
				$.set_attribute(input, "state", $.get(state));
				$.set_value(input, count());
			});
			$.append($$anchor, fragment_1);
		};
		$.add_svelte_meta(() => $.if(node_4, ($$render) => {
			if ($.get(state)) $$render(consequent);
			else $$render(alternate_1, -1);
		}), "if", App, 214, 8);
	}
	$.reset(div_2);
	$.bind_this(div_2, ($$value) => dynamicEl = $$value, () => dynamicEl);
	var node_6 = $.sibling(div_2, 2);
	$.add_svelte_meta(() => $.key(node_6, () => $.get(counter), ($$anchor) => {
		var p_3 = root_11();
		var text_11 = $.child(p_3);
		$.reset(p_3);
		$.template_effect(() => $.set_text(text_11, `Keyed content chunk 0: ${$.get(counter) ?? ""}`));
		$.transition(3, p_3, () => slide);
		$.append($$anchor, p_3);
	}), "key", App, 236, 4);
	var node_7 = $.sibling(node_6, 2);
	$.add_svelte_meta(() => $.each(node_7, 27, items, (item) => item.id, ($$anchor, item, idx) => {
		const itemLabel = $.tag($.derived(() => `${$.get(idx)}:${$.get(item).name}`), "itemLabel");
		$.get(itemLabel);
		var p_4 = root_12();
		$.attribute_effect(p_4, () => ({
			...rest,
			"data-index": `chunk-0-${$.get(idx) ?? ""}`
		}), void 0, void 0, void 0, "svelte-13nvtxg");
		var text_12 = $.child(p_4, true);
		$.reset(p_4);
		$.template_effect(() => $.set_text(text_12, $.get(itemLabel)));
		$.animation(p_4, () => flip, null);
		$.append($$anchor, p_4);
	}), "each", App, 240, 4);
	var node_8 = $.sibling(node_7, 2);
	$.add_svelte_meta(() => $.each(node_8, 17, items, $.index, ($$anchor, $$item) => {
		var span_3 = root_13();
		$.append($$anchor, span_3);
	}), "each", App, 245, 4);
	var node_9 = $.sibling(node_8, 2);
	$.add_svelte_meta(() => $.await(node_9, () => promise, ($$anchor) => {
		var p_7 = root_16();
		$.append($$anchor, p_7);
	}, ($$anchor, value) => {
		var p_5 = root_14();
		var text_13 = $.child(p_5);
		$.reset(p_5);
		$.template_effect(() => $.set_text(text_13, `Resolved: ${$.get(value) ?? ""}`));
		$.append($$anchor, p_5);
	}, ($$anchor, error) => {
		var p_6 = root_15();
		var text_14 = $.child(p_6);
		$.reset(p_6);
		$.template_effect(() => $.set_text(text_14, `Error: ${$.get(error).message ?? ""}`));
		$.append($$anchor, p_6);
	}), "await", App, 249, 4);
	var node_10 = $.sibling(node_9, 2);
	$.add_svelte_meta(() => $.await(node_10, () => promise, null, ($$anchor, quickValue) => {
		var p_8 = root_17();
		var text_15 = $.child(p_8);
		$.reset(p_8);
		$.template_effect(() => $.set_text(text_15, `Quick resolved: ${$.get(quickValue) ?? ""}`));
		$.append($$anchor, p_8);
	}), "await", App, 257, 4);
	var input_1 = $.sibling(node_10, 2);
	$.remove_input_defaults(input_1);
	var textarea = $.sibling(input_1, 2);
	$.remove_textarea_child(textarea);
	var select = $.sibling(textarea, 2);
	var option = $.child(select);
	option.value = option.__value = "opt-0";
	var option_1 = $.sibling(option);
	option_1.value = option_1.__value = "opt-1";
	$.reset(select);
	var input_2 = $.sibling(select, 2);
	$.remove_input_defaults(input_2);
	var input_3 = $.sibling(input_2, 2);
	$.remove_input_defaults(input_3);
	input_3.value = input_3.__value = "opt-0";
	var div_4 = $.sibling(input_3, 2);
	$.bind_this(div_4, ($$value) => inputEl = $$value, () => inputEl);
	var video = $.sibling(div_4, 2);
	var div_5 = $.sibling(video, 2);
	$.action(div_5, ($$node, $$action_arg) => action?.($$node, $$action_arg), () => $.get(state));
	var div_6 = $.sibling(div_5, 2);
	var div_7 = $.sibling(div_6, 2);
	var node_11 = $.sibling(div_7, 2);
	{
		$.validate_dynamic_element_tag(() => $.get(state) ? "div" : "span");
		$.validate_void_dynamic_element(() => $.get(state) ? "div" : "span");
		$.element(node_11, () => $.get(state) ? "div" : "span", false, ($$element, $$anchor) => {
			$.set_class($$element, 0, "dynamic-0 svelte-13nvtxg");
			var text_16 = $.text();
			$.template_effect(() => $.set_text(text_16, `Dynamic element chunk 0: ${title() ?? ""}`));
			$.append($$anchor, text_16);
		}, void 0, [275, 4]);
	}
	var node_12 = $.sibling(node_11, 2);
	{
		let $0 = $.derived(getHandler);
		$.add_svelte_meta(() => $.bind_this(ChildComponent(node_12, {
			get title() {
				return title();
			},
			get onclick() {
				return $.get($0);
			},
			children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
				var strong = root_19();
				var text_17 = $.child(strong);
				$.reset(strong);
				$.template_effect(() => $.set_text(text_17, `Inline child chunk 0: ${title() ?? ""}`));
				$.append($$anchor, strong);
			}),
			$$slots: {
				default: true,
				footer: ($$anchor, $$slotProps) => {
					var div_8 = root_20();
					var text_18 = $.child(div_8);
					$.reset(div_8);
					$.template_effect(() => $.set_text(text_18, `Footer chunk 0: ${$.get(counter) ?? ""}`));
					$.append($$anchor, div_8);
				}
			}
		}), ($$value) => componentRef = $$value, () => componentRef), "component", App, 279, 4, { componentTag: "ChildComponent" });
	}
	var node_13 = $.sibling(node_12, 2);
	$.add_svelte_meta(() => badge(node_13, () => "chunk-0", () => "secondary"), "render", App, 284, 4);
	var node_14 = $.sibling(node_13, 2);
	$.add_svelte_meta(() => card(node_14, title, () => "Content for chunk 0"), "render", App, 285, 4);
	var node_15 = $.sibling(node_14, 2);
	$.add_svelte_meta(() => metricSummary(node_15, () => ({
		label: title(),
		values: [
			count(),
			$.get(doubled),
			$.get(counter)
		],
		meta: { id: propsId }
	})), "render", App, 286, 4);
	var node_16 = $.sibling(node_15, 2);
	$.add_svelte_meta(() => show?.(node_16), "render", App, 287, 4);
	var button = $.sibling(node_16, 2);
	var p_9 = $.sibling(button, 2);
	var text_19 = $.child(p_9);
	$.reset(p_9);
	var node_17 = $.sibling(p_9, 2);
	{
		const failed = $.wrap_snippet(App, function($$anchor, error = $.noop) {
			$.validate_snippet_args(...arguments);
			var p_10 = root_21();
			var text_20 = $.child(p_10);
			$.reset(p_10);
			$.template_effect(() => $.set_text(text_20, `Error in chunk 0: ${error().message ?? ""}`));
			$.append($$anchor, p_10);
		});
		$.boundary(node_17, {
			onerror: handleError,
			failed
		}, ($$anchor) => {
			var p_11 = root_22();
			var text_21 = $.child(p_11);
			$.reset(p_11);
			$.template_effect(() => $.set_text(text_21, `Boundary chunk 0: ${title() ?? ""}`));
			$.append($$anchor, p_11);
		});
	}
	$.reset(div_1);
	$.template_effect(() => {
		$.set_text(text_6, `Chunk 0: Lorem ${$.get(state) ?? ""} + ${$.get(state) ?? ""} = Ipsum; `);
		$.set_text(text_7, `Props: title=${title() ?? ""}, count=${count() ?? ""}, doubled=${$.get(doubled) ?? ""}, computed=${$.get(computed) ?? ""}`);
		$.set_text(text_8, `Module: ${$.get(moduleSummary) ?? ""} | Store: ${$.get(storeSummary) ?? ""} | Label: ${$labelStore() ?? ""}`);
		classes_1 = $.set_class(div_2, 1, $.clsx({
			active: $.get(checked),
			big: $.get(counter) > 10
		}), "svelte-13nvtxg", classes_1, {
			state: $.get(state),
			staticly: true,
			invinsible,
			reactive: $.get(counter)
		});
		styles = $.set_style(div_2, "", styles, {
			color: $.get(state),
			"font-size": "14px",
			opacity: $.get(counter) / 100,
			"--custom": "value-0"
		});
		$.set_text(text_19, `Metric count: ${$metrics().length ?? ""}`);
	});
	$.delegated("click", div_2, handleClick);
	$.event("scroll", div_2, handleClick);
	$.event("click", div_2, handleClick, true);
	$.event("focus", div_2, function(...$$args) {
		$.apply(() => $.get(event_handler), this, $$args, App, [208, 17], true, true);
	});
	$.bind_value(input_1, function get() {
		return $.get(state);
	}, function set($$value) {
		$.set(state, $$value);
	});
	$.bind_value(textarea, function get() {
		return $.get(state);
	}, function set($$value) {
		$.set(state, $$value);
	});
	$.bind_select_value(select, function get() {
		return $.get(selected);
	}, function set($$value) {
		$.set(selected, $$value);
	});
	$.bind_checked(input_2, function get() {
		return $.get(checked);
	}, function set($$value) {
		$.set(checked, $$value);
	});
	$.bind_group(binding_group, [], input_3, function get() {
		return $.get(group);
	}, function set($$value) {
		$.set(group, $$value);
	});
	$.bind_element_size(div_4, "clientWidth", function set($$value) {
		$.set(counter, $$value);
	});
	$.bind_content_editable("innerHTML", div_4, function get() {
		return $.get(state);
	}, function set($$value) {
		$.set(state, $$value);
	});
	$.bind_volume(video, function get() {
		return $.get(volume);
	}, function set($$value) {
		$.set(volume, $$value);
	});
	$.bind_paused(video, function get() {
		return $.get(checked);
	}, function set($$value) {
		$.set(checked, $$value);
	});
	$.transition(3, div_6, () => fade);
	$.transition(1, div_7, () => fly, () => ({ y: 200 }));
	$.transition(2, div_7, () => fade);
	$.delegated("click", button, addMetric);
	$.append($$anchor, div_1);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
$.delegate(["click"]);
$.create_custom_element(App, {
	title: {},
	count: {},
	items: {},
	config: {},
	multiplier: {},
	visible: {}
}, [], ["APP_VERSION", "formatTitle"], { mode: "open" });
