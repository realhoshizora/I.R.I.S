---
version: 1
slug: "src-popup-html"
primary_target: "src/popup.html"
related_targets: ["src/overlay.html","src/fab.html","src/settings.html"]
---

# Surface: translation popup, region overlay, selection button, settings

Mode: Operate. Users read Japanese/English content on Windows and want Vietnamese meaning without leaving it. Task: highlight or box text, read the result, dismiss. Constraint: never cover the selected text. Chosen by the user from a 3-way preview (A. bilingual subtitle).

## Direction contract

THESIS: A translation is a subtitle track laid under what you are reading. Refuses the category default of a white Fluent card with a language dropdown and a grey translated paragraph.

OWN-WORLD: Opaque near-black band (#0e1116), 6px corners, soft drop shadow. Source line small in 78% white; translation in subtitle yellow (#f7d44c) with a hairline black outline shadow. Languages are colored tracks: JA vermilion (#ef6a4c), EN blue (#6cb4ff), VI yellow. Be Vietnam Pro for Vietnamese/Latin (bundled locally), Yu Gothic UI / Meiryo for Japanese (ship with Windows).

STORY: The user sees which track was detected, reads the source once, reads the Vietnamese, copies or dismisses with Esc.

FIRST VIEWPORT: Popup 420px wide placed below the selection (above it when there is no room). Top row: JA → VI track chips left, copy / pin / close right. Then source line, then translation. Single words: headword 24px + reading, then part-of-speech rows (label in muted caps, meaning in yellow).

FORM: Bilingual fansub subtitle, candidate 6 of 7 on the ordered list; seed:d3dd4d9a148aa415. Signature move: track chips plus line-by-line cue reveal of the result.

FINISH: unreviewed and undocumented is unfinished; this build ends with the finish review, the verdict, DESIGN.md, and every shipping raster carrying its provenance
