\title{{\huge\sffamily AOSC DeployKit}\linebreak Product Requirement Document}
\author{Neruthes}
\date{2022-02-10 v0.0.0-pre2}

\maketitle

\setcounter{tocdepth}{1}
\textsf{\tableofcontents}

\vspace{30pt}
\hrule
\vspace{15pt}


# Abstract

This document describes the feature requirements for DeployKit, a software which is designed to assist the installation of AOSC OS.


# Introduction

AOSC OS has a long history of manual installation, like Arch Linux or Gentoo.
However, manual installation is considered unfriendly for new users,
especially when some distributions like Ubuntu have much smoother installation experience.
Therefore, the community decided to create a friendly installer program, under the name `DeployKit` or `aoscdk-rs`.


## Working Environment

### Architecture

DeployKit shall work on the `amd64` architecture.
If possible, it will be nice to have it compatible with other mainline architectures (e.g. `arm64`).

Compatibility with Retro architectures (e.g. `ppc64el`) will be a surprise, and not an expectation.

### Software

DeployKit shall work in a generic LiveCD environment, inside a generic interactive shell (hopefully `bash`).
This program shall work in a TUI fashion and shall not require any GUI.


## Usability Objectives

- The user only needs basic command-line knowledge.
- The user can get links to the documentation when in doubt, hopefully by scanning a QR code.
