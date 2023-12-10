<script>
    import Card from "../elements/Card.svelte";
    import iMiniLink from "../icons/link1.svg";
</script>

<div class="px-5 py-2">
    <div class="mb-5">
        <h1 class="font-bold text-2xl">anykit</h1>
        Lorem ipsum dolor sit amet consectetur adipisicing elit. Ad, officia?
    </div>
    <div class="flex flex-wrap gap-7">
        <Card title="Mini link" link="/minilink" icon={iMiniLink} />
    </div>
</div>
