<script lang="ts">
    export let title: string;
    export let icon: string;
    export let link: string;
</script>

<a class="card-element" href={link}>
    <label class="lcard-elem" for="header">{title}</label>
    <div class="h-full w-full">
        <img class="kit-elem" src={icon} alt="icon" />
    </div>
</a>
