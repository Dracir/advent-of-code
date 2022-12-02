Based on
https://colin-fraser.net/post/a-quick-tutorial-on-importing-data-from-advent-of-code-into-r/

# steps
Set ~/.bashsrc variables

    Open a terminal - CTRL+Alt+T.
    Edit file ~/.bashsrc by

nano ~/.bashsrc

    Add the variable to the bottom of the file:
        export RSTUDIO_PANDOC=/usr/lib/rstudio/bin/pandoc
    Save and close nano.
    Apply changes by:

source ~/.bashsrc
