<script lang="ts">
  import type { Snippet } from 'svelte';

  let {
    icon,
    title,
    iconColor = '',
    class: className = '',
    headerRight,
    children
  }: {
    icon?: string;
    title?: string;
    iconColor?: string;
    class?: string;
    headerRight?: Snippet;
    children?: Snippet;
  } = $props();
</script>

<div class="p-3.5 sm:p-5 rounded-2xl bg-zinc-900/60 border border-zinc-800/80 hover:border-zinc-700/60 transition-all shadow-sm flex flex-col justify-between min-w-0 {className}">
  {#if icon || title || headerRight}
    <div class="flex items-center justify-between gap-2 shrink-0 min-w-0">
      <div class="flex items-center gap-1.5 min-w-0">
        {#if icon}
          <span class="text-sm shrink-0 {iconColor}">{icon}</span>
        {/if}
        {#if title}
          <span class="text-xs font-semibold text-zinc-200 truncate">{title}</span>
        {/if}
      </div>

      {#if headerRight}
        <div class="shrink-0 flex items-center">
          {@render headerRight()}
        </div>
      {/if}
    </div>
  {/if}

  {#if children}
    <div class="{icon || title || headerRight ? 'mt-3 sm:mt-3.5' : ''} space-y-3 flex-1 flex flex-col justify-between min-w-0">
      {@render children()}
    </div>
  {/if}
</div>
