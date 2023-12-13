<script lang="ts">
    import KitHeader from "../../elements/KitHeader.svelte";
    import iLink from "../../icons/link2.svg";
    import iQrCode from "../../icons/qrcode.svg";
    import iCopy from "../../icons/copy.svg";

    import { invoke } from "@tauri-apps/api/tauri";

    let inputLink: string;
    let shortLink: string;

    const get_link = async () => {
        shortLink = await invoke("short_link", { url: inputLink });
    };

    const get_qrcode = () => {
        shortLink = "";
    };

    const copyClipboard = (value: string) => {
        navigator.clipboard.writeText(value);
    };
</script>

<div class="px-5 py-2">
    <KitHeader title="Mini Link" desc="Shorten your link" />
    <div class="mb-8">
        <label class="block text-slate-300 font-bold mb-1 text-xl" for="link"
            >Link</label
        >
        <div class="flex gap-1 mb-10">
            <input
                bind:value={inputLink}
                class="px-2 rounded-sm bg-slate-800 w-full h-10 text-lg"
                type="text"
                placeholder="Put your link here"
            />
            <button
                on:click={get_link}
                class="px-2 bg-slate-800 hover:bg-slate-800/50 w-20 h-10 text-lg rounded-sm"
            >
                <img class="m-auto" src={iLink} alt="icon" />
            </button>
            <button
                on:click={get_qrcode}
                class="px-2 bg-slate-800 hover:bg-slate-800/50 w-20 h-10 text-lg rounded-sm"
            >
                <img class="m-auto" src={iQrCode} alt="icon" />
            </button>
        </div>
    </div>

    {#if shortLink}
        <div class="flex gap-1 mb-10 justify-center">
            <input
                bind:value={shortLink}
                class="px-2 rounded-sm text-slate-600 bg-slate-800/50 w-1/2 h-10 text-lg"
                type="text"
                disabled
            />
            <button
                on:click={() => copyClipboard(shortLink)}
                class="px-2 bg-slate-800 hover:bg-slate-800/50 w-20 h-10 text-lg rounded-sm"
            >
                <img class="m-auto" src={iCopy} alt="icon" />
            </button>
        </div>
    {/if}
    <div id="qrcode"></div>
</div>
