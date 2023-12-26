<script>
    import KitHeader from "../../elements/KitHeader.svelte";
</script>

<KitHeader title="Tubelo" desc="download youtube video from link" />
<label class="block text-slate-300 font-bold mb-1 text-xl" for="link"
    >Link</label
>
<div class="flex gap-1 mb-10">
    <input
        class="px-2 rounded-sm bg-slate-800 w-full h-10 text-lg"
        type="text"
        placeholder="Put your link here"
    />
</div>
<label class="block text-slate-300 font-bold mb-2 text-xl" for="link"
    >Download</label
>
<button class="px-2 bg-slate-800 hover:bg-slate-800/50 h-10 text-lg rounded-sm">
    MP 3
</button>
<button class="px-2 bg-slate-800 hover:bg-slate-800/50 h-10 text-lg rounded-sm">
    MP 4
</button>
