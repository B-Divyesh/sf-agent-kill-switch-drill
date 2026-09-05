# Agent Kill-Switch Drill — visual system

## Direction

**Dithered / halftone incident printout.** The product is about an operational
rehearsal that needs to be understood in seconds under pressure. The site takes
its cue from a field manual and a risograph incident card: hard black type,
paper ground, a vermilion stop signal, and a deliberately imperfect dot screen.
The texture is functional rather than decorative: it separates the simulated
control-plane chain from the explanatory copy and makes the report feel like an
artifact a team would save.

## Tokens

| Token | Value | Use |
| --- | --- | --- |
| paper | `#f7f1e5` | page background |
| ink | `#172125` | primary text, nav, outlines |
| muted | `#526066` | supporting text |
| oxide | `#b33220` | stop / destructive control and emphasis |
| leaf | `#1d634c` | confirmed / pass state |
| amber | `#875c10` | rehearsal / caution state |
| night | `#10181b` | dark theme background |
| night-paper | `#f6f1e5` | dark theme text |

The dark treatment preserves the printed-operations tone rather than inverting
to a generic blue dashboard. It uses a dedicated near-black drill surface,
light card paper, warm high-contrast labels, and a darker vermilion action
fill. This avoids the old variable inversion that made the drill card unreadable.
All body text pairs meet 4.5:1 contrast; interactive controls and outlines meet
the 3:1 UI threshold.

## Type and spacing

The title uses the local system **Georgia** stack for a briefing-sheet editorial
voice. Interface and command text use the local system **ui-monospace** stack,
which makes command IDs and result timestamps reliably scannable without any
network font request. Scale runs 12 / 14 / 16 / 20 / 28 / 52 px. Layout follows
a 4 px rhythm, with 16 px small gaps, 24 px module gaps, and 48–72 px section
breaks. Wide copy remains below 70 characters.

## Interaction and motion

Controls are pressable stamped labels with a 2 px ink outline and a 2 px
translation on activation. The simulated incident card fills one stage at a
time (180 ms opacity + transform); results never rely on color alone. With
`prefers-reduced-motion`, stage changes are instant and the background dot
texture stays still. The phone view removes the decorative torn-paper edge and
stacks controls before the card.

## Asset plan and provenance

`site/public/relay-drum.webp` and its 640 px responsive companion are original,
generated illustrations: a
mechanical emergency relay drum, severed capability threads, and halftone
paper texture. It will be generated with the factory image deployment via
`/opt/fleet/lib/gen-image.sh`, optimised locally to WebP, and used only as the
hero illustration. Prompt: “editorial risograph illustration of a mechanical
emergency relay drum that cuts three labeled-but-unreadable tool authority
threads, dark ink line work, vermilion stop button, forest green confirmation
lights, warm recycled paper ground, coarse halftone dots, no words, no logos,
no watermark, landscape composition.” The deployment produced a PNG that was
locally resized and encoded as 1280 px WebP (268 KB) and 640 px WebP (45 KB).
The asset is an original generated work; no third-party assets or fonts are
loaded. `site/public/og-card.webp` is a local 1200×630 crop of that original
illustration for sharing metadata, and `site/public/apple-touch-icon.png` is a
local 180×180 crop. Neither adds a third-party asset.
