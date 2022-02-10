#!/bin/bash

PROJDIR="$(dirname $(realpath "$0"))"
PROJNAME="$(basename "$PROJDIR")"

PANDOC_LATEX_VARS="
-V fontsize=11pt
-V papersize:a4
-V geometry:margin=30mm
"


for DIRPATH in $PROJDIR/*; do
    if [[ -d $DIRPATH ]]; then
        DIRNAME="$(basename "$DIRPATH")"
        echo "[INFO] Building document $DIRPATH"
        cat     "$PROJDIR/$DIRNAME"/*.md \
                "$PROJDIR/.footer.tex" \
        | pandoc -s \
            $PANDOC_LATEX_VARS \
            -f markdown -t pdf \
            --pdf-engine=xelatex \
            --number-sections \
            -o "$PROJDIR/$DIRNAME.pdf"
    fi
done

