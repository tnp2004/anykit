<script>
    import Card from "../elements/Card.svelte";
    import Loader from "../elements/Loader.svelte";
    import iMiniLink from "../icons/link1.svg";
    import iTubelo from "../icons/tubelo.svg";
</script>

<div class="mb-5">
    <h1 class="font-bold text-2xl">anykit</h1>
    Lorem ipsum dolor sit amet consectetur adipisicing elit. Ad, officia?
</div>
<div class="flex flex-wrap gap-7">
    <Card title="MiniLink" link="/minilink" icon={iMiniLink} />
    <Card title="Tubelo" link="/tubelo" icon={iTubelo} />
</div>
