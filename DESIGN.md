---
name: I.R.I.S
description: On-screen translation laid under what you are reading, like a subtitle track.
colors:
  band: "#0e1116"
  band-raised: "#171b22"
  line: "#2a313c"
  line-hover: "#3a4350"
  ink: "#ffffff"
  ink-2: "rgba(255, 255, 255, 0.78)"
  ink-3: "#8f98a3"
  subtitle-yellow: "#f7d44c"
  track-ja: "#ef6a4c"
  track-en: "#6cb4ff"
  track-vi: "#f7d44c"
  track-other: "#a3acb7"
  chip-ink: "#111418"
  err: "#ff9b8a"
typography:
  headword:
    fontFamily: "Be Vietnam Pro, Segoe UI, system-ui, sans-serif"
    fontSize: "24px"
    fontWeight: 700
    lineHeight: 1.25
  subtitle:
    fontFamily: "Be Vietnam Pro, Segoe UI, system-ui, sans-serif"
    fontSize: "18px"
    fontWeight: 600
    lineHeight: 1.42
  title:
    fontFamily: "Be Vietnam Pro, Segoe UI, system-ui, sans-serif"
    fontSize: "18px"
    fontWeight: 700
    letterSpacing: "0.02em"
  meaning:
    fontFamily: "Be Vietnam Pro, Segoe UI, system-ui, sans-serif"
    fontSize: "15px"
    fontWeight: 600
  body:
    fontFamily: "Be Vietnam Pro, Segoe UI, system-ui, sans-serif"
    fontSize: "14px"
    fontWeight: 400
    lineHeight: 1.5
  body-ja:
    fontFamily: "Yu Gothic UI, Meiryo UI, Meiryo, Be Vietnam Pro, sans-serif"
    fontSize: "14px"
    fontWeight: 400
    lineHeight: 1.5
  label:
    fontFamily: "Be Vietnam Pro, Segoe UI, system-ui, sans-serif"
    fontSize: "13px"
    fontWeight: 600
  caps-label:
    fontFamily: "Be Vietnam Pro, Segoe UI, system-ui, sans-serif"
    fontSize: "11px"
    fontWeight: 600
    letterSpacing: "0.05em"
  track:
    fontFamily: "Be Vietnam Pro, Segoe UI, system-ui, sans-serif"
    fontSize: "11px"
    fontWeight: 600
    lineHeight: 1
    letterSpacing: "0.06em"
rounded:
  chip: "3px"
  control-sm: "4px"
  control: "5px"
  band: "6px"
spacing:
  xs: "4px"
  sm: "6px"
  md: "10px"
  lg: "14px"
  xl: "24px"
components:
  track-chip:
    backgroundColor: "{colors.track-other}"
    textColor: "{colors.chip-ink}"
    typography: "{typography.track}"
    rounded: "{rounded.chip}"
    padding: "4px 6px"
  track-chip-ja:
    backgroundColor: "{colors.track-ja}"
  track-chip-en:
    backgroundColor: "{colors.track-en}"
  track-chip-vi:
    backgroundColor: "{colors.track-vi}"
  band-popup:
    backgroundColor: "{colors.band}"
    rounded: "{rounded.band}"
    padding: "10px 12px 14px 14px"
    width: "420px"
  icon-button:
    textColor: "{colors.ink-3}"
    rounded: "{rounded.control}"
    size: "28px"
  icon-button-hover:
    backgroundColor: "rgba(255, 255, 255, 0.07)"
    textColor: "{colors.ink}"
  icon-button-pressed:
    textColor: "{colors.subtitle-yellow}"
  button-primary:
    backgroundColor: "{colors.subtitle-yellow}"
    textColor: "{colors.band}"
    typography: "{typography.label}"
    rounded: "{rounded.control}"
    padding: "9px 18px"
  button-ghost:
    textColor: "{colors.ink-2}"
    typography: "{typography.label}"
    rounded: "{rounded.control}"
    padding: "0 12px"
  button-ghost-hover:
    textColor: "{colors.ink}"
  input:
    backgroundColor: "{colors.band-raised}"
    textColor: "{colors.ink}"
    typography: "{typography.body}"
    rounded: "{rounded.control}"
    padding: "8px 10px"
  kbd:
    backgroundColor: "{colors.band-raised}"
    textColor: "{colors.ink}"
    rounded: "{rounded.control-sm}"
    padding: "2px 7px"
---

# Design System: I.R.I.S

## Overview

**Creative North Star: "The Subtitle Track"**

A translation is a subtitle laid under what the user is reading. Every surface is an opaque near-black band floating over someone else's app: the source line sits small in dimmed white, the translation reads in subtitle yellow with a hairline black outline, and languages are named by coloured track chips, the way a fansub labels its tracks. The world refuses the category default of a white card with a language dropdown and a grey translated paragraph.

Density is compact and glanceable; the popup is read mid-task and dismissed with Esc. There is one look, dark, by design: the band is a subtitle band, not a theme, so there is no light variant. Latin and Vietnamese set in Be Vietnam Pro (bundled, OFL); Japanese falls to the Windows faces Yu Gothic UI and Meiryo through `:lang(ja)`.

**Key Characteristics:**
- Opaque near-black band, 6px corners, soft ambient shadow.
- Translation in subtitle yellow with a 1px black outline shadow; source in 78% white.
- Coloured language track chips are the signature: JA vermilion, EN blue, VI yellow.
- Result lines reveal cue by cue (fade and 3px rise, 80ms stagger).
- Thin-stroke 16px line icons in a quiet muted grey until hovered.

## Colors

A near-black band carrying one warm voice, subtitle yellow, plus three track colours that exist only to name languages.

### Primary
- **Subtitle Yellow** (subtitle-yellow): the translation text, dictionary meanings, settings section headings, the focus ring, checkbox accent, selection tint (35% alpha), the region box stroke, the pressed state of icon buttons, and the fill of the primary action (Save, Retry).

### Tertiary (language tracks)
- **Vermilion Track** (track-ja): Japanese chip fill.
- **Broadcast Blue Track** (track-en): English chip fill.
- **Yellow Track** (track-vi): Vietnamese chip fill; same value as subtitle yellow, because Vietnamese is the subtitle.
- **Unknown Track Grey** (track-other): chip fill for undetected or other languages.
- **Chip Ink** (chip-ink): the dark label on every chip.

### Neutral
- **Subtitle Band** (band): the background of every surface: popup, region hint, selection button, settings window.
- **Raised Band** (band-raised): inputs, selects and `kbd` keys inside the band.
- **Hairline** (line): 1px dividers and control borders; **Hairline Hover** (line-hover) on hover.
- **White Ink** (ink): headwords, labels, primary text.
- **Dim Ink** (ink-2): the source line, readings, secondary text.
- **Muted Ink** (ink-3): engine name, hints, part-of-speech labels, resting icons, pending state.
- **Soft Coral** (err): error text on the band.

### Named Rules
**The One Voice Rule.** Subtitle yellow is the only accent. It marks the translation and the one action that matters; nothing decorative is yellow.

**The Tracks Name Languages Rule.** Vermilion, blue and grey appear only as language track chips. They never colour text, borders or buttons.

## Typography

**Body Font:** Be Vietnam Pro (with Segoe UI, system-ui)
**Japanese Font:** Yu Gothic UI (with Meiryo UI, Meiryo), applied by `:lang(ja)`

**Character:** One humanist sans with full Vietnamese diacritic coverage carries everything Latin; Japanese uses the faces Windows already ships, so kana and kanji render natively and nothing is downloaded.

### Hierarchy
- **Headword** (700, 24px, 1.25): the looked-up word in a dictionary result.
- **Subtitle** (600, 18px, 1.42): the translation line, in subtitle yellow with outline shadow.
- **Title** (700, 18px, 0.02em): settings window title.
- **Meaning** (600, 15px): dictionary meanings, in yellow. The region hint uses the same 15px/600 step.
- **Body** (400, 14px, 1.5): source line, form values, default text.
- **Label** (600, 13px): form labels, buttons, `kbd`.
- **Caps Label** (600, 11px, 0.05em, uppercase): part-of-speech labels in muted ink.
- **Track** (600, 11px, 1, 0.06em): track chip text.

### Named Rules
**The Readable Subtitle Rule.** The translation is always the largest weighted text in a sentence result (18px/600) and always carries the outline shadow (`0 1px 0 #000, 0 0 2px #000`); the source above it stays at 14px in dim ink.

## Layout

The popup is a fixed 420px band inside a transparent 10/12/14px frame that leaves room for its shadow, placed below the selection (above it when there is no room). Inside: a top bar (source chip → target chip, engine name, then copy / pin / close pushed right with 2px gaps), then the cue stack. The cue stack caps at 360px (and shrinks to fit the screen), scrolls with contained overscroll, and fades its bottom 20px when more content is below. Dictionary results use a two-column grid (label auto, meaning 1fr; 6px × 12px gaps).

Settings is a single column: header with title and the three track chips right, sections separated by hairlines, 24px side padding, hints capped at 60ch, and a sticky footer holding the Save button.

Spacing runs in small steps: 4, 6, 10, 14 and 24px, with 10px the usual gap between cue lines.

## Elevation & Depth

Bands float over another app, so they carry soft ambient shadow; nothing inside a band is elevated. Depth inside a surface is tonal (band → raised band) and hairline, never shadow.

### Shadow Vocabulary
- **Popup band** (`box-shadow: 0 8px 24px rgba(0,0,0,0.38), 0 1px 3px rgba(0,0,0,0.3)`): the translation popup.
- **Floating hint** (`box-shadow: 0 6px 18px rgba(0,0,0,0.35)`): the region-select instruction.
- **Selection button** (`box-shadow: 0 4px 12px rgba(0,0,0,0.35)`): the small translate button.
- **Subtitle outline** (`text-shadow: 0 1px 0 #000, 0 0 2px #000`): translation text only; the region hint uses `0 1px 0 #000`.

### Named Rules
**The Floating Band Rule.** Shadows belong to the outer band only, scaled to its size; controls inside it stay flat.

## Shapes

Small, tight corners: 3px on chips, 4–5px on controls and keys, 6px on bands. The region box is a 2px subtitle-yellow stroke with 2px corners, and the rest of the screen is dimmed by a 42% near-black scrim. Borders are 1px hairlines; `kbd` keys add a 2px bottom edge.

## Components

### Track Chip (signature)
- **Style:** 11px/600 uppercase-code label (JA, EN, VI) in chip ink on its track colour, 4px 6px padding, 3px corners, minimum 26px wide.
- **Use:** the popup bar (source → target), the selection button, the settings header.

### Buttons
- **Icon button:** 28px square, transparent, muted ink, 5px corners, 16px line icon (1.7 stroke, round caps). Hover: white ink on 7% white. Pressed (pin): subtitle yellow. Disabled: 35% opacity.
- **Primary:** subtitle-yellow fill with band-coloured text, 5px corners (Retry uses 4px and 6px 12px padding). Hover: `brightness(1.08)`. Disabled: 60% opacity.
- **Ghost:** transparent with hairline border, dim ink; hover lifts to white ink and hover hairline.
- **Selection button:** 34px band square, 6px corners, holding the target track chip; pops in with a 0.18s scale from 0.85.

### Inputs / Fields
- **Style:** raised band fill, 1px hairline, 5px corners, 8px 10px padding, 14px text.
- **Focus:** the global 2px subtitle-yellow outline at 2px offset.
- **Checkbox:** native, 16px, subtitle-yellow accent.

### Cue Reveal
Result lines enter one by one: 0.32s `cubic-bezier(0.16, 1, 0.3, 1)` from 20% opacity and 3px down, staggered 80ms. Reduced motion turns off every animation and transition.

## Do's and Don'ts

### Do:
- **Do** set every surface on the opaque subtitle band with 6px corners.
- **Do** put the translation in subtitle yellow at 18px/600 with the black outline shadow.
- **Do** name languages only with track chips (JA vermilion, EN blue, VI yellow, other grey).
- **Do** mark Japanese runs with `lang="ja"` so they fall to Yu Gothic UI / Meiryo.
- **Do** keep icons as 16px inline SVG line icons at 1.7 stroke in `currentColor`.

### Don't:
- **Don't** add a light theme or a translucent/acrylic band; the band is opaque and dark by design.
- **Don't** fall back to a white card with a language dropdown and a grey translated paragraph.
- **Don't** use track colours outside chips, or yellow for decoration.
- **Don't** shadow controls inside a band.
