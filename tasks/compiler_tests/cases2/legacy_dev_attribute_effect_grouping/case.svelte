<script>
    let title = "t";
    let counter = 0;
    let flag = "x";

    function bump() {
        title = title + "!";
        counter += 1;
        flag = flag + "!";
    }
</script>

<button onclick={bump}>bump</button>
<div title={title} data-counter={counter} data-flag={flag}></div>
