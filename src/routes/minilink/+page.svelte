<script lang="ts">
    import BackBtn from "../../elements/BackBtn.svelte";
    import iLink from "../../icons/link2.svg";
    import iQrCode from "../../icons/qrcode.svg";
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/tauri";
</script>

<div class="px-5 py-2">
    <div class="flex mb-10">
        <BackBtn />
        <div class="w-full text-center mr-[40px]">
            <h1 class="text-2xl font-bold">Mini Link</h1>
        </div>
    </div>

    <div class="mb-8">
        <label class="block text-slate-300 font-bold" for="description"
            >Description</label
        >
        <p class="text-slate-300">
            Lorem ipsum dolor sit amet consectetur adipisicing elit. Fuga vel
            temporibus ullam inventore ipsa expedita blanditiis neque veniam
            autem iusto.
        </p>
    </div>

    <div class="mb-8">
        <label class="block text-slate-300 font-bold mb-1 text-xl" for="link"
            >Link</label
        >
        <div class="flex gap-1 mb-10">
            <input
                class="px-2 rounded-sm bg-slate-800 w-full h-10 text-lg"
                type="text"
                placeholder="Put your link here"
            />
            <button
                class="px-2 bg-slate-800 hover:bg-slate-800/50 w-20 h-10 text-lg rounded-sm"
            >
                <img class="m-auto" src={iLink} alt="icon" />
            </button>
            <button
                class="px-2 bg-slate-800 hover:bg-slate-800/50 w-20 h-10 text-lg rounded-sm"
            >
                <img class="m-auto" src={iQrCode} alt="icon" />
            </button>
            
        </div>
    </div>
</div>
