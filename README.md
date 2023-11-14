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

| Year | Game                                                                                        | Platform | Version | Format     | Status |
|------|---------------------------------------------------------------------------------------------|----------|---------|------------|--------|
| 2002 | Jimmy Neutron: Boy Genius - BigSky                                                          |          |         | Kalisto    | ✔      |
|      | SpongeBob SquarePants: Revenge of the Flying Dutchman - BigSky                              |          |         | Kalisto    | ✔      |
|      | Spirits & Spells (Castleween) (Mahou no Pumpkin) - Wanadoo                                  |          |         | Kalisto    | ✔      |
| 2003 | Super Farm - Asobo                                                                          |          |         | Asobo1     | ✔      |
| 2004 | Sitting Ducks - Asobo                                                                       |          |         | Asobo1     | ✔      |
|      | The Mummy: The Animated Series - Asobo                                                      |          |         | Asobo1     | ✔      |
| 2005 | CT Special Forces: Fire for Effect (Nemesis Strike) - Asobo                                 |          |         | Asobo2     | ✔      |
|      | Ratatouille (Prototype) - Asobo                                                             |          |         | Asobo2     | ✔      |
| 2006 | Garfield: A Tail of Two Kitties (Garfield 2) - Asobo                                        |          |         | Asobo2     | ✔      |
|      | Championsheep Rally - Black Sheep                                                           |          |         | BlackSheep | ✔      |
| 2007 | Ratatouille - Asobo                                                                         |          |         | Asobo3     | ✔      |
|      | The Ugly Duckling and Me - Black Sheep                                                      |          |         | BlackSheep | ✔      |
|      | En Taxi avec Oui-Oui - Black Sheep                                                          |          |         | BlackSheep | ✔      |
| 2008 | WALL-E - Asobo                                                                              |          |         | Asobo3     | ✔      |
|      | The Magic Roundabout - Black Sheep                                                          |          |         | BlackSheep | ✔      |
|      | Shaun White Snowboarding/Shaun White Snowboarding: Road Trip (Prototype) - Ubisoft Montreal |          |         | Ubisoft1   | ✔      |
|      | Shaun White Snowboarding/Shaun White Snowboarding: Road Trip - Ubisoft Montreal             |          |         | Ubisoft2   | ✔      |
|      | Warning: Code De La Route - Black Sheep                                                     |          |         | BlackSheep | ✔      |
| 2009 | FUEL - Asobo                                                                                |          |         | Asobo3     | ✔      |
|      | Up - Asobo                                                                                  |          |         | Asobo3     | ✔      |
|      | Shaun White Snowboarding: World Stage - Ubisoft Montreal                                    |          |         | Ubisoft2   | ✔      |
| 2010 | Toy Story 3 - Asobo                                                                         |          |         | Asobo3     | ✔      |
|      | Racket Sports/Racquet Sports/Racket Sports Party - Asobo                                    |          |         | Asobo3     | ✔      |
|      | Happy Neuron Academy - Black Sheep                                                          |          |         | BlackSheep | ✔      |
| 2012 | Kinect Rush: A Disney-Pixar Adventure - Asobo                                               |          |         | Asobo4     | ✔      |
| 2013 | Super Farm (Re-release) - Asobo                                                             |          |         | Asobo4     | ✔      |
| 2014 | Monopoly Plus/Monopoly Deal - Asobo                                                         |          |         | Asobo4     | ✔      |
| 2015 | The Mighty Quest for Epic Loot - Ubisoft Montreal                                           |          |         | Ubisoft3   | ✔      |
| 2016 | Young Conker - Asobo                                                                        |          |         | Asobo4     | ✔      |
|      | Fragments - Asobo                                                                           |          |         | Asobo4     | ✔      |
| 2017 | Rush: A Disney-Pixar Adventure (Re-release) - Asobo                                         |          |         | Asobo5     | ✔      |
|      | Monopoly Plus/Monopoly Deal/Monopoly for Nintendo Switch (Re-release) - Asobo               |          |         | Asobo4     | ✔      |
| 2019 | A Plague Tale: Innocence - Asobo                                                            |          |         | Asobo6     | ⚠️     |
| 2020 | Microsoft Flight Simulator - Asobo                                                          |          |         | Asobo7     | ❌      |
| 2022 | A Plague Tale: Requiem - Asobo                                                              |          |         | Asobo8     | ❌      |

### TSC

| Format | Status |
|--------|--------|
| csc    | ✔      |
| psc    | ✔      |
| CPS    | ❌      |

These formats from Black Sheep Studios games also use the csc cypher and therefore work with the csc command. They aren't actually TSCs, but they are text based configuration formats, so I'll include them here.

| Format | Status |
|--------|--------|
| cmf    | ✔      |
| cgf    | ✔      |
| cst    | ✔      |

### Audio

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

The [ImZouna](https://github.com/widberg/ImZouna) and [zouna-templates-docs](https://github.com/SabeMP/zouna-templates-docs) repositories have binary patterns for Zouna data structures.

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
