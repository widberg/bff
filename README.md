# bff

BigFile Friend

[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/widberg/bff/build.yml)](https://github.com/widberg/bff/actions)
[![Release Nightly](https://img.shields.io/badge/release-nightly-5e025f?labelColor=301934)](https://nightly.link/widberg/bff/workflows/build/master)
[![Join the chat at https://discord.gg/CQgMNbYeUR](https://img.shields.io/badge/chat-on_discord-7389D8.svg?logo=discord&logoColor=ffffff&labelColor=6A7EC2)](https://discord.gg/CQgMNbYeUR)

The Zouna Swiss Army knife. Originally named BigFile Friend for Zouna's resource archives, it now supports far more than just BigFiles.

<sup>This repository is a relative of the main [FMTK repository](https://github.com/widberg/fmtk).</sup>

## Support

A ✔ indicates that the format has been tested and is working. An ❌ indicates that the format is not yet implemented. A ⚠️ indicates partial/incomplete support.

### BigFile

| Year | Game                                                                                        | Status |
|------|---------------------------------------------------------------------------------------------|--------|
| 2002 | Jimmy Neutron: Boy Genius - BigSky                                                          | ✔      |
|      | SpongeBob SquarePants: Revenge of the Flying Dutchman - BigSky                              | ✔      |
|      | Spirits & Spells (Castleween) (Mahou no Pumpkin) - Wanadoo                                  | ✔      |
| 2003 | Super Farm - Asobo                                                                          | ✔      |
| 2004 | Sitting Ducks - Asobo                                                                       | ✔      |
|      | The Mummy: The Animated Series - Asobo                                                      | ✔      |
| 2005 | CT Special Forces: Fire for Effect (Nemesis Strike) - Asobo                                 | ✔      |
|      | Ratatouille (Prototype) - Asobo                                                             | ✔      |
| 2006 | Garfield: A Tail of Two Kitties (Garfield 2) - Asobo                                        | ✔      |
|      | Championsheep Rally - Black Sheep                                                           | ✔      |
| 2007 | Ratatouille - Asobo                                                                         | ✔      |
|      | The Ugly Duckling and Me - Black Sheep                                                      | ✔      |
|      | En Taxi avec Oui-Oui - Black Sheep                                                          | ✔      |
| 2008 | WALL-E - Asobo                                                                              | ✔      |
|      | The Magic Roundabout - Black Sheep                                                          | ✔      |
|      | Shaun White Snowboarding/Shaun White Snowboarding: Road Trip (Prototype) - Ubisoft Montreal | ✔      |
|      | Shaun White Snowboarding/Shaun White Snowboarding: Road Trip - Ubisoft Montreal             | ✔      |
|      | Warning: Code De La Route - Black Sheep                                                     | ✔      |
| 2009 | FUEL - Asobo                                                                                | ✔      |
|      | Up - Asobo                                                                                  | ✔      |
|      | Shaun White Snowboarding: World Stage - Ubisoft Montreal                                    | ✔      |
| 2010 | Toy Story 3 - Asobo                                                                         | ✔      |
|      | Racket Sports/Racquet Sports/Racket Sports Party - Asobo                                    | ✔      |
|      | Happy Neuron Academy - Black Sheep                                                          | ✔      |
| 2012 | Kinect Rush: A Disney-Pixar Adventure - Asobo                                               | ✔      |
| 2013 | Super Farm (Re-release) - Asobo                                                             | ✔      |
| 2014 | Monopoly Plus/Monopoly Deal - Asobo                                                         | ✔      |
| 2015 | The Mighty Quest for Epic Loot - Ubisoft Montreal                                           | ✔      |
| 2016 | Young Conker - Asobo                                                                        | ✔      |
|      | Fragments - Asobo                                                                           | ✔      |
| 2017 | Rush: A Disney-Pixar Adventure (Re-release) - Asobo                                         | ✔      |
|      | Monopoly Plus/Monopoly Deal/Monopoly for Nintendo Switch (Re-release) - Asobo               | ✔      |
| 2019 | A Plague Tale: Innocence - Asobo                                                            | ⚠️      |
| 2020 | Microsoft Flight Simulator - Asobo                                                          | ⚠️      |
| 2022 | A Plague Tale: Requiem - Asobo                                                              | ⚠️      |
| 2024 | Microsoft Flight Simulator 2024 - Asobo                                                     | ❌      |

### TSC

| Format | Status |
|--------|--------|
| csc    | ✔      |
| psc    | ✔      |
| CPS    | ✔      |

These formats from Black Sheep Studios games also use the csc cypher and therefore work with the csc command. They aren't actually TSCs, but they are text based configuration formats, so I'll include them here.

| Format | Status |
|--------|--------|
| cmf    | ✔      |
| cgf    | ✔      |
| cst    | ✔      |

This format is from The Mighty Quest for Epic Loot. Again, it's not actually a TSC file, but it is an encrypted and compressed archive format used for storing JSON configuration files, so I'll include it here.

| Format          | Status |
|-----------------|--------|
| settings.bin    | ❌     |

### Audio

These formats are low priority since they can all be played using the instructions on the [FMTK Wiki Asobo Audio Formats page](https://github.com/widberg/fmtk/wiki/Asobo-Audio-Formats). The implementation of these formats is being tracked in [issue #27](https://github.com/widberg/bff/issues/27).

| Format  | Status |
|---------|--------|
| SoundBF | ❌      |
| AIF     | ❌      |
| JOE     | ❌      |
| VAI     | ❌      |

### Archive

| Format  | Status |
|---------|--------|
| FAT+LIN | ✔      |

## Patterns

The [ImZouna](https://github.com/widberg/ImZouna) and [zouna-templates-docs](https://github.com/SabeMP/zouna-templates-docs) repositories have binary patterns for Zouna data structures. Even formats that are not supported by bff have patterns in these repositories.

## Showcase

These cool projects use bff. Join the [Zouna Underground Discord](https://discord.gg/CQgMNbYeUR) or open an issue to let me know if you made something cool and I'll add it here.

### `ratatouille_converter.py`

Ahmed Khaled's [`ratatouille_converter.py`](https://gist.github.com/widberg/2abbbca02b532104bd32cc27743fa9f6#file-ratatouille_converter-py) script converts BMFONT FNT XML files to bff Fonts_Z JSON files.

## Getting Started

### Prerequisites

* [Rust](https://www.rust-lang.org/)

### Checkout

```sh
git clone https://github.com/widberg/bff.git
cd bff
```

### Build

```sh
cargo build --release
```

### Test

```sh
RUST_TEST_THREADS=1 cargo +nightly test --release -j 1
```
