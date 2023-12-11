<script lang="ts">
    import BackBtn from "./BackBtn.svelte";

    export let title: string;
    export let desc: string;
</script>

<div class="flex mb-10">
    <BackBtn />
    <div class="w-full text-center mr-[40px]">
        <h1 class="text-2xl font-bold">{title}</h1>
    </div>
</div>

<div class="mb-8">
    <label class="block text-slate-300 font-bold" for="description"
        >Description</label
    >
    <p class="text-slate-300">
       {desc}
    </p>
</div>